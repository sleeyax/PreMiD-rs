#![allow(non_snake_case)]
use std::{
    path::PathBuf,
    sync::{mpsc::channel, Arc},
};

use axum::routing::get;
use axum::Server;
use clap::{arg, command, value_parser};
use serde_json::Value;
use socketioxide::{Namespace, SocketIoLayer};

use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use crate::{
    constants::{APP_VERSION, DEFAULT_ADDRESS, REPO_URL},
    local_presence::{LocalFile, LocalFiles, LocalPresence},
    log::setup_logger,
    rpc_client::RpcClient,
    rpc_client_manager::RpcClientManager,
    settings::Settings,
    types::Presence,
};

mod constants;
mod local_presence;
mod log;
mod rpc_client;
mod rpc_client_manager;
mod settings;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    let matches = command!()
        .arg(
            arg!(
                -l --local <DIR> "Path to local presence dist folder"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let (tx, rx) = channel::<Vec<LocalFile>>();

    if let Some(local) = matches.get_one::<PathBuf>("local").cloned() {
        info!("Using local presence: {}", local.display());
        tokio::task::spawn_blocking(move || {
            let local_presence = LocalPresence::new();
            local_presence.watch_files(local.as_path(), tx).unwrap();
        });
    }

    let manager = Arc::new(Mutex::new(RpcClientManager::new()));
    let rx = Arc::new(Mutex::new(rx));

    let ns = Namespace::builder()
        .add("/", move |socket| {
            debug!("Socket connected with id: {}", socket.sid);

            let manager_clone = Arc::clone(&manager);
            let rx_clone = Arc::clone(&rx);

            async move {
                socket.on("getVersion", |socket, data: Value, bin, _| async move {
                    debug!("getVersion: {:?} {:?}", data, bin);
                    socket
                        .bin(bin)
                        .emit("receiveVersion", APP_VERSION.to_string())
                        .ok();
                });

                socket.on("settingUpdate", |_, data: Settings, bin, _| async move {
                    debug!("settingUpdate: {:?} {:?}", data, bin);
                });

                socket.on("selectLocalPresence", |_, data: Value, bin, _| async move {
                    debug!("selectLocalPresence: {:?} {:?}", data, bin);
                    warn!("Local presence is not supported at runtime. Please launch PreMiD-rs and specify a path to your local presence with the flag '--local <path/to/dist>'.");
                });

                let manager = Arc::clone(&manager_clone);
                socket.on("setActivity", move |_, data: Presence, bin, _| {
                    debug!("setActivity: {:?} {:?}", data, bin);

                    let manager = Arc::clone(&manager);

                    async move {
                        tokio::spawn(async move {
                            let mut manager = manager.lock().await;

                            if let Some(hidden) = data.hidden {
                                if hidden {
                                    if let Some(rpc_client) =
                                        manager.get_client_mut(data.client_id.clone())
                                    {
                                        rpc_client.clear_activity();
                                    }
                                }
                            }

                            if let Some(rpc_client) = manager.get_client_mut(data.client_id.clone())
                            {
                                rpc_client.set_activity(data);
                            } else {
                                let mut rpc_client = RpcClient::new(data.client_id.clone());
                                rpc_client.set_activity(data);
                                manager.add_client(rpc_client);
                            }
                        });
                    }
                });

                let manager = Arc::clone(&manager_clone);
                socket.on("clearActivity", move |_, data: Value, bin, _| {
                    debug!("clearActivity: {:?} {:?}", data, bin);

                    let manager = Arc::clone(&manager);

                    async move {
                        tokio::spawn(async move {
                            let mut manager = manager.lock().await;
                            manager.clear_all_activities();
                        });
                    }
                });

                tokio::spawn(async move {
                    let rpc_client = RpcClient::default();
                    socket.emit("discordUser", rpc_client.get_user()).ok();
                    drop(rpc_client);

                    let rx = rx_clone.lock().await;
                    for files in rx.iter() {
                        info!("localPresence: local files changed");
                        socket.emit("localPresence", LocalFiles::new(files)).ok();
                    }
                });
            }
        })
        .build();

    let app = axum::Router::new()
        .route(
            "/",
            get(|| async {
                format!(
                    "PreMiD-rs server is running. Go to {} for help and support.",
                    REPO_URL
                )
            }),
        )
        .layer(SocketIoLayer::new(ns));

    info!("Starting server on http://{}", DEFAULT_ADDRESS);

    Server::bind(&DEFAULT_ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
