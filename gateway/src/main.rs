// fn main() {
//     let i = Ini::load_from_file("/Users/shiroki/Downloads/DefaultPalWorldSettings.ini").unwrap();
//     for (sec, prop) in i.iter() {
//         println!("Section: {:?}", sec);
//         for (k, v) in prop.iter() {
//             // println!("{}:{}", k, v);
//             if v.starts_with('(') && v.ends_with(')') {
//                 // assume as Unreal config struct
//                 println!("{k}: {:?}", gateway_rs::unreal_struct::parse_struct(v));
//             } else {
//                 println!("{k}: {v}");
//             }
//         }
//     }
// }

use axum::{
    extract::{State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use palboard_gateway::pal::{PalServerClient, PalworldCommandError};
use serde::Deserialize;
use serde_json::json;
use std::env;
use thiserror::Error;
use tracing::{info, warn};

const VERSION: Option<&str> = option_env!("VERSION");

#[derive(Error, Debug)]
enum AppError {
    #[error("error from the inner RCON client")]
    PalworldCommandError(#[from] PalworldCommandError),
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

async fn update_steam_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(palboard_gateway::steamcmd::update_steam)
}

#[tokio::main]
async fn main() {
    console_subscriber::init();
    // tracing_subscriber::fmt::init();

    let client = {
        let mut c = PalServerClient::dial(
            env::var("PALSERVER_ADDR").expect("you should set `PALSERVER_ADDR` (and optionally `PALSERVER_PASSWORD`) environment variable"), 
            env::var("PALSERVER_PASSWORD").ok())
            .await
            .expect("failed to dial Pal Server");
        let info = c.info().await.expect("failed to get info");
        info!("Client dial succeeded: {}", info.trim());
        c
    };

    let app = Router::new()
        .route("/version", get(VERSION.unwrap_or("unknown")))
        .nest(
            "/pal",
            Router::new()
        .route("/shutdown", post(shutdown_handler))
        .route("/exit", post(exit_handler))
        .route("/broadcast", post(broadcast_handler))
        .route("/kick", post(kick_handler))
        .route("/ban", post(ban_handler))
        .route("/players", get(players_handler))
        .route("/info", get(info_handler))
        .route("/save", post(save_handler))
                .with_state(client),
        )
        .nest(
            "/steam",
            Router::new().route("/update", get(update_steam_handler)),
        );

    let listener = tokio::net::TcpListener::bind(env::var("GATEWAY_ADDR").unwrap_or_else(|_| {
        warn!("you should set `GATEWAY_ADDR` environment variable, frontend will connect to this address");
        "127.0.0.1:8080".to_string()
    }))
    .await
    .unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            #[cfg(unix)]
            {
                use tokio::signal::unix::{signal, SignalKind};
                let mut sigint = signal(SignalKind::interrupt()).unwrap();
                let mut sigterm = signal(SignalKind::terminate()).unwrap();
                tokio::select! {
                    _ = sigint.recv() => info!("SIGINT"),
                    _ = sigterm.recv() => info!("SIGTERM")
                }
            }
            #[cfg(not(unix))]
            {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to install CTRL+C signal handler");
            }
        })
        .await
        .unwrap();
}
