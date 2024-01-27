use bytes::{Buf, BytesMut};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::net::ToSocketAddrs;
use tokio_util::codec::{Decoder, Encoder};
use tracing::instrument;

#[derive(Debug)]
struct RCONCodec;

#[derive(Debug)]
struct RCONMessage {
    // size is u32 but only in the wire
    pub id: i32,
    pub kind: i32,
    pub body: String,
}

const SERVERDATA_AUTH: i32 = 3;
const SERVERDATA_AUTH_RESPONSE: i32 = 2;
const SERVERDATA_EXECCOMMAND: i32 = 2;
const SERVERDATA_RESPONSE_VALUE: i32 = 0;

impl Decoder for RCONCodec {
    type Item = RCONMessage;
    type Error = std::io::Error; // TODO: use custom error type

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }
        let length = {
            let mut buf = [0u8; 4];
            (&src[0..4]).copy_to_slice(&mut buf);
            u32::from_le_bytes(buf) as usize
        };
        if length > 4096 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", length),
            ));
        }
        if src.len() < 4 + length {
            src.reserve(4 + length - src.len());
            return Ok(None);
        }
        src.advance(4);
        let id = {
            let mut buf = [0u8; 4];
            src.copy_to_slice(&mut buf);
            i32::from_le_bytes(buf)
        };
        let kind = {
            let mut buf = [0u8; 4];
            src.copy_to_slice(&mut buf);
            i32::from_le_bytes(buf)
        };
        let body = {
            let mut buf = vec![0u8; length - 8 - 2];
            src.copy_to_slice(&mut buf);
            String::from_utf8(buf)
        };
        src.advance(2);

        match body {
            Ok(body) => Ok(Some(RCONMessage { id, kind, body })),
            Err(utf8_error) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                utf8_error.utf8_error(),
            )),
        }
    }
}

impl Encoder<RCONMessage> for RCONCodec {
    type Error = std::io::Error; // TODO: use custom error type

    fn encode(&mut self, item: RCONMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if item.body.as_bytes().len() + 10 > 4096 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Frame of length {} is too large.",
                    item.body.as_bytes().len() + 10
                ),
            ));
        }

        let body = item.body.as_bytes();
        let length = 4 + 4 + 4 + body.len() + 2;
        dst.reserve(length);
        dst.extend_from_slice(&(length as u32).to_le_bytes());
        dst.extend_from_slice(&item.id.to_le_bytes());
        dst.extend_from_slice(&item.kind.to_le_bytes());
        dst.extend_from_slice(body);
        dst.extend_from_slice(&[0u8; 2]);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RCONClient {
    conn: tokio_util::codec::Framed<tokio::net::TcpStream, RCONCodec>,
}

impl RCONClient {
    #[instrument(skip_all, err(Debug))]
    pub async fn dial(
        addr: impl ToSocketAddrs,
        password: Option<impl ToString>,
    ) -> tokio::io::Result<Self> {
        let stream = tokio::net::TcpStream::connect(addr).await?;
        let codec = RCONCodec;
        let mut conn = codec.framed(stream);

        if let Some(password) = password {
            conn.send(RCONMessage {
                id: 13,
                kind: SERVERDATA_AUTH,
                body: password.to_string(),
            })
            .await?;
            let response = conn.next().await.ok_or(tokio::io::Error::new(
                tokio::io::ErrorKind::ConnectionReset,
                "Connection closed unexpectedly",
            ))??;
            if response.id != 13 || response.kind != SERVERDATA_AUTH_RESPONSE {
                return Err(tokio::io::Error::new(
                    tokio::io::ErrorKind::InvalidData,
                    format!(
                        "Invalid response to auth request: {:?}, probably wrong password",
                        response
                    ),
                ));
            }
        }

        Ok(RCONClient { conn })
    }
    pub async fn exec(&mut self, command: impl ToString) -> tokio::io::Result<String> {
        self.conn
            .send(RCONMessage {
                id: 14,
                kind: SERVERDATA_EXECCOMMAND,
                body: command.to_string(),
            })
            .await?;
        let response = self.conn.next().await.ok_or(tokio::io::Error::new(
            tokio::io::ErrorKind::ConnectionReset,
            "Connection closed unexpectedly",
        ))??;
        if response.kind != SERVERDATA_RESPONSE_VALUE {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::InvalidData,
                format!("Invalid response to exec request: {:?}", response),
            ));
        }
        Ok(response.body)
    }
}
