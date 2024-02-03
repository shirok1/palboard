use std::{sync::Arc, time::Duration};
use tokio::{
    net::ToSocketAddrs,
    sync::{mpsc, oneshot},
    time::timeout,
};

use crate::rcon::RCONClient;
use thiserror::Error;
use tracing::instrument;

pub mod route;

#[derive(Debug, Clone)]
pub struct PalServerClient {
    tx: mpsc::Sender<(String, oneshot::Sender<tokio::io::Result<String>>)>,
    _runner: Arc<tokio::task::JoinHandle<()>>,
}

#[derive(Error, Debug)]
pub enum PalworldCommandError {
    #[error("error from the inner RCON client")]
    RCONError(#[from] tokio::io::Error),
    #[error("the mpsc channel rx used to receive the command was dropped")]
    RunnerDroppedCommandRx(mpsc::error::SendError<CommandReciple>),
    #[error("the oneshot channel tx used to return the result was dropped")]
    RunnerDroppedReturnTx(oneshot::error::RecvError),
}

type CommandReciple = (String, oneshot::Sender<tokio::io::Result<String>>);
type PalResult<T> = std::result::Result<T, PalworldCommandError>;

impl PalServerClient {
    #[instrument(skip_all)]
    async fn task_runner(mut client: RCONClient, mut command_rx: mpsc::Receiver<CommandReciple>) {
        loop {
            match timeout(Duration::from_secs(5), command_rx.recv()).await {
                Ok(None) => break,
                Ok(Some((command, tx))) => {
                    let result = client.exec(command).await;
                    if tx.send(result).is_err() {
                        tracing::error!("failed to send result back");
                        break;
                    };
                }
                Err(_) => {
                    // timeout, send a keepalive
                    let res = client.exec("ShowPlayers").await;
                    if res.is_err() {
                        tracing::error!("failed to send keepalive");
                        break;
                    }
                }
            }
        }
    }
    pub async fn dial(
        addr: impl ToSocketAddrs,
        password: Option<impl ToString>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = RCONClient::dial(addr, password).await?;
        let (tx, rx) = mpsc::channel(32);
        let _runner = Arc::new(tokio::spawn(Self::task_runner(client, rx)));
        Ok(Self { tx, _runner })
    }
    async fn exec(&mut self, command: String) -> Result<String, PalworldCommandError> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send((command, tx))
            .await
            .map_err(PalworldCommandError::RunnerDroppedCommandRx)?;

        Ok(rx
            .await
            .map_err(PalworldCommandError::RunnerDroppedReturnTx)??)
    }
    pub async fn shutdown(
        &mut self,
        seconds: usize,
        message_text: impl ToString,
    ) -> PalResult<String> {
        self.exec(format!("Shutdown {} {}", seconds, message_text.to_string()))
            .await
    }
    pub async fn do_exit(&mut self) -> PalResult<String> {
        self.exec("DoExit".to_string()).await
    }
    pub async fn broadcast(&mut self, message_text: impl ToString) -> PalResult<String> {
        self.exec(format!("Broadcast {}", message_text.to_string()))
            .await
    }
    pub async fn kick_player(&mut self, steamid: impl ToString) -> PalResult<String> {
        self.exec(format!("KickPlayer {}", steamid.to_string()))
            .await
    }
    pub async fn ban_player(&mut self, steamid: impl ToString) -> PalResult<String> {
        self.exec(format!("BanPlayer {}", steamid.to_string()))
            .await
    }
    pub async fn show_players(&mut self) -> PalResult<String> {
        self.exec("ShowPlayers".to_string()).await
    }
    pub async fn info(&mut self) -> PalResult<String> {
        self.exec("Info".to_string()).await
    }
    pub async fn save(&mut self) -> PalResult<String> {
        self.exec("Save".to_string()).await
    }
}
