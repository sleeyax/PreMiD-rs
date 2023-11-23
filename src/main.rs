#![allow(non_snake_case)]
use std::sync::Arc;

use axum::routing::get;
use axum::Server;
use serde_json::Value;
use socketioxide::{
    extract::{Bin, Data, SocketRef},
    SocketIo,
};

use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::{
    constants::{APP_VERSION, DEFAULT_ADDRESS, REPO_URL},
    log::setup_logger,
    rpc_client::RpcClient,
    rpc_client_manager::RpcClientManager,
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

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", move |socket: SocketRef, _: Data<Value>| {
        debug!("Socket connected with id: {}", socket.id);

        let manager_clone = Arc::clone(&manager);

        async move {
            socket.on(
                "getVersion",
                |socket: SocketRef, Data::<Value>(data), Bin(bin)| async move {
                    debug!("getVersion: {:?} {:?}", data, bin);
                    socket
                        .bin(bin)
                        .emit("receiveVersion", APP_VERSION.to_string())
                        .ok();
                },
            );

            socket.on(
                "settingUpdate",
                |_: SocketRef, Data::<Value>(data), Bin(bin)| async move {
                    debug!("settingUpdate: {:?} {:?}", data, bin);
                },
            );

            let manager = Arc::clone(&manager_clone);
            socket.on(
                "setActivity",
                move |_: SocketRef, Data::<Presence>(data), Bin(bin)| {
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
                },
            );

            let manager = Arc::clone(&manager_clone);
            socket.on(
                "clearActivity",
                move |_: SocketRef, Data::<Value>(data), Bin(bin)| {
                    debug!("clearActivity: {:?} {:?}", data, bin);

                    let manager = Arc::clone(&manager);

                    async move {
                        tokio::spawn(async move {
                            let mut manager = manager.lock().await;
                            manager.clear_all_activities();
                        });
                    }
                },
            );

            tokio::spawn(async move {
                let rpc_client = RpcClient::default();
                socket.emit("discordUser", rpc_client.get_user()).ok();
                drop(rpc_client);
            });
        }
    });

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
        .layer(layer);

    Server::bind(&DEFAULT_ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
