use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::AppResult;

use super::PalServerClient;

pub fn new_router(client: PalServerClient) -> Router<()> {
    Router::new()
        .route("/shutdown", post(shutdown_handler))
        .route("/exit", post(exit_handler))
        .route("/broadcast", post(broadcast_handler))
        .route("/kick", post(kick_handler))
        .route("/ban", post(ban_handler))
        .route("/players", get(players_handler))
        .route("/info", get(info_handler))
        .route("/save", post(save_handler))
        .with_state(client)
}

async fn info_handler(State(mut c): State<PalServerClient>) -> AppResult<impl IntoResponse> {
    let body = c.info().await?;
    Ok(body)
}

async fn players_handler(State(mut c): State<PalServerClient>) -> AppResult<impl IntoResponse> {
    let body = c.show_players().await?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(body.as_bytes());
    let vec = rdr
        .records()
        .flatten()
        .flat_map(|rec| {
            Some(json!({
                "name": rec.get(0)?,
                "playeruid": rec.get(1)?,
                "steamid": rec.get(2)?,
            }))
        })
        .collect::<Vec<_>>();
    Ok(Json(vec))
}

#[derive(Deserialize)]
struct BroadcastRequest {
    message: String,
}

async fn broadcast_handler(
    State(mut c): State<PalServerClient>,
    Json(req): Json<BroadcastRequest>,
) -> AppResult<impl IntoResponse> {
    Ok(c.broadcast(req.message).await?)
}

#[derive(Deserialize)]
struct ShutdownRequest {
    time: usize,
    message: String,
}
async fn shutdown_handler(
    State(mut c): State<PalServerClient>,
    Json(req): Json<ShutdownRequest>,
) -> AppResult<impl IntoResponse> {
    Ok(c.shutdown(req.time, req.message).await?)
}
async fn exit_handler(State(mut c): State<PalServerClient>) -> AppResult<impl IntoResponse> {
    Ok(c.do_exit().await?)
}

#[derive(Deserialize)]
struct KickOrBanRequest {
    steamid: String,
}
async fn kick_handler(
    State(mut c): State<PalServerClient>,
    Json(req): Json<KickOrBanRequest>,
) -> AppResult<impl IntoResponse> {
    Ok(c.kick_player(req.steamid).await?)
}
async fn ban_handler(
    State(mut c): State<PalServerClient>,
    Json(req): Json<KickOrBanRequest>,
) -> AppResult<impl IntoResponse> {
    Ok(c.ban_player(req.steamid).await?)
}

async fn save_handler(State(mut c): State<PalServerClient>) -> AppResult<impl IntoResponse> {
    Ok(c.save().await?)
}
