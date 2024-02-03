use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub mod pal;
pub mod rcon;
pub mod steamcmd;
pub mod unreal_struct;

#[derive(Error, Debug)]
enum AppError {
    #[error("error from the inner RCON client")]
    PalworldCommandError(#[from] pal::PalworldCommandError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {:?}", self),
        )
            .into_response()
    }
}

type AppResult<T> = std::result::Result<T, AppError>;
