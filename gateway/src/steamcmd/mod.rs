use thiserror::Error;
use tokio::process::{Child, ChildStdout, Command};
use tokio_util::io::ReaderStream;
use tracing::error;

use std::ffi::OsStr;
use std::process::Stdio;

pub mod route;

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
fn update_args_for(update_type: UpdateType) -> &'static [&'static str] {
    match update_type {
        UpdateType::Steam => STEAMCMD_UPDATE_ARGS,
        UpdateType::Game { validate: true } => STEAMCMD_UPDATE_GAME_ARGS,
        UpdateType::Game { .. } => STEAMCMD_UPDATE_GAME_NO_VALIDATE_ARGS,
    }
}
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
