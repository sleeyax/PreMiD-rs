#![allow(non_snake_case)]
use std::sync::Arc;

use axum::routing::get;
use axum::Server;
use serde_json::Value;
use socketioxide::{Namespace, SocketIoLayer};

use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::{
    constants::{APP_VERSION, DEFAULT_ADDRESS, REPO_URL},
    log::setup_logger,
    rpc_client::RpcClient,
    rpc_client_manager::RpcClientManager,
    settings::Settings,
    types::Presence,
};

mod constants;
mod log;
mod rpc_client;
mod rpc_client_manager;
mod settings;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    info!("Starting server on http://{}", DEFAULT_ADDRESS);

    let manager = Arc::new(Mutex::new(RpcClientManager::new()));

    let ns = Namespace::builder()
        .add("/", move |socket| {
            debug!("Socket connected with id: {}", socket.sid);

            let manager_clone = Arc::clone(&manager);

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
                    socket
                        .emit("discordUser", RpcClient::default().get_user())
                        .ok();
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

    Server::bind(&DEFAULT_ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
