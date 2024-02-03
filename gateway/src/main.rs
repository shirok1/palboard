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

use axum::{routing::get, Router};
use palboard_gateway::{
    pal::{self, PalServerClient},
    steamcmd,
};
use std::env;
use tracing::{info, warn};

const VERSION: Option<&str> = option_env!("VERSION");

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
        .nest("/pal", pal::route::new_router(client))
        .nest("/steam", steamcmd::route::new_router());

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
