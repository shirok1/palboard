use axum::extract::ws::WebSocket;
use serde::Serialize;
use thiserror::Error;
use tokio::{
    io::AsyncBufReadExt,
    process::{Child, ChildStdout, Command},
    spawn,
    sync::{mpsc, Mutex},
};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tokio_util::io::{ReaderStream, StreamReader};
use tracing::{debug, debug_span, error, instrument, Instrument};

use std::ffi::OsStr;
use std::process::Stdio;
use std::sync::Arc;

const STEAMCMD_EXE: &str = "/home/steam/steamcmd/steamcmd.sh"; // as in cm2network/steamcmd
#[derive(Debug)]
pub enum UpdateType {
    Steam,
    Game { validate: bool },
}
const STEAMCMD_UPDATE_ARGS: &[&str] = &["+login", "anonymous", "+quit"];
const STEAMCMD_UPDATE_GAME_ARGS: &[&str] = &[
    "+force_install_dir",
    "/home/steam/palserver",
    "+login",
    "anonymous",
    "+app_update",
    "2394010",
    "validate",
    "+quit",
];
const STEAMCMD_UPDATE_GAME_NO_VALIDATE_ARGS: &[&str] = &[
    "+force_install_dir",
    "/home/steam/palserver",
    "+login",
    "anonymous",
    "+app_update",
    "2394010",
    "+quit",
];
const BUFFER_SIZE: usize = 128;

#[derive(Error, Debug)]
pub enum SteamCMDError {
    #[error("during spawn")]
    SpawnError(std::io::Error),
}

type SteamCMDResult<T> = Result<T, SteamCMDError>;

pub async fn run_steamcmd(
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> SteamCMDResult<(Child, ReaderStream<ChildStdout>)> {
    let mut child = Command::new("/bin/stdbuf")
        .arg("--output=0")
        .arg(STEAMCMD_EXE)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(SteamCMDError::SpawnError)?;

    drop(child.stdin.take());

    let stdout = ReaderStream::new(child.stdout.take().unwrap());

    Ok((child, stdout))
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum UpdateSteamMessage {
    SteamSelfUpdate {
        status: String,
    },
    UpdateState {
        state_id: u32,
        state_name: String,
        progress: String,
        current: u64,
        total: u64,
    },
    Success,
    Error {
        reason: String,
    },
}

fn parse_line(line: &str) -> Option<UpdateSteamMessage> {
    // TODO: reusing regexes
    let update_state_pattern = regex::Regex::new(r"^ Update state \(0x(?<state_id>[\da-f]+)\) (?<state_name>[\w ]+), progress: (?<progress>\d*\.\d*) \((?<current>\d+) \/ (?<total>\d+)\)$").unwrap();
    let steam_self_update_pattern = regex::Regex::new(r"^\[....\] (.+)$").unwrap();
    let error_pattern = regex::Regex::new(r"^ERROR!.+\((.+)\)$").unwrap();

    if line.starts_with("Success!") {
        return Some(UpdateSteamMessage::Success);
    }

    if let Some(cap) = steam_self_update_pattern.captures(&line) {
        let (_, [status]) = cap.extract();
        let status = status.to_string();
        return Some(UpdateSteamMessage::SteamSelfUpdate { status });
    }

    if let Some(cap) = error_pattern.captures(&line) {
        let (_, [reason]) = cap.extract();
        let reason = reason.to_string();
        return Some(UpdateSteamMessage::Error { reason });
    }

    if let Some(cap) = update_state_pattern.captures(&line) {
        let (_, [state_id, state_name, progress, current, total]) = cap.extract();
        let state_id = u32::from_str_radix(state_id, 16).unwrap();
        let current = u64::from_str_radix(current, 10).unwrap();
        let total = u64::from_str_radix(total, 10).unwrap();
        let state_name = state_name.to_string();
        let progress = progress.to_string();
        return Some(UpdateSteamMessage::UpdateState {
            state_id,
            state_name,
            progress,
            current,
            total,
        });
    }

    None
}

#[instrument(skip_all)]
pub async fn update_steam(ws: WebSocket, update_type: UpdateType) {
    let (mut child, mut stdout) = run_steamcmd(match update_type {
        UpdateType::Steam => STEAMCMD_UPDATE_ARGS,
        UpdateType::Game { validate: true } => STEAMCMD_UPDATE_GAME_ARGS,
        UpdateType::Game { .. } => STEAMCMD_UPDATE_GAME_NO_VALIDATE_ARGS,
    })
    .await
    .unwrap();

    let (tx, rx) = mpsc::channel(BUFFER_SIZE);

    let rx = ReceiverStream::new(rx);
    let mut lines = StreamReader::new(rx).lines();

    let ws = Arc::new(Mutex::new(ws));
    let ws_lr = ws.clone();

    let line_reader = spawn(
        async move {
            while let Some(line) = lines.next_line().await.unwrap() {
                debug!("parsing line: {}", line);

                if let Some(msg) = parse_line(&line) {
                    ws_lr
                        .lock()
                        .await
                        .send(axum::extract::ws::Message::Text(
                            serde_json::to_string(&msg).unwrap(),
                        ))
                        .await
                        .unwrap();
                }
            }
            debug!("exit");
        }
        .instrument(debug_span!("line_reader")),
    );

    while let Some(chunk) = stdout.next().await {
        match chunk {
            Ok(data) => {
                tx.send(Ok(data.clone())).await.unwrap();
                debug!("piping data to user: {:?}", data);
                let res = ws
                    .lock()
                    .await
                    .send(axum::extract::ws::Message::Binary(data.to_vec()))
                    .await;
                debug!("pipe result: {:?}", res);
            }
            Err(err) => {
                let _ = tx
                    .send(Err(std::io::Error::new(err.kind(), err.to_string())))
                    .await;
                error!("while piping:: {}", err);
                break;
            }
        }
    }

    debug!("waiting for child exit");
    let res = child.wait().await;
    debug!("child exit ret {:?}", res);

    drop(tx);
    line_reader.await.unwrap();
    debug!("line_reader joined");

    ws.lock()
        .await
        .send(axum::extract::ws::Message::Close(None))
        .await
        .unwrap();
}
