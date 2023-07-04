use axum::routing::get;
use axum::Server;
use serde_json::Value;
use socketioxide::{Namespace, SocketIoLayer};
use std::{
    default,
    sync::{Arc, Mutex},
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{
    constants::{APP_VERSION, DEFAULT_ADDRESS, DEFAULT_CLIENT_ID, REPO_URL},
    rpc_client::RpcClient,
    rpc_client_manager::RpcClientManager,
    settings::Settings,
    types::Presence,
};

mod constants;
mod rpc_client;
mod rpc_client_manager;
mod settings;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let rpc_client_manager = Arc::new(Mutex::new(RpcClientManager::new()));

    info!("Starting server on http://{}", DEFAULT_ADDRESS);

    let rpc_client_manager_clone = Arc::clone(&rpc_client_manager);
    let ns = Namespace::builder()
        .add("/", move |socket| {
            let rpc_client_manager = Arc::clone(&rpc_client_manager_clone);

            async move {
                println!("Socket connected with id: {}", socket.sid);

                socket.on("getVersion", |socket, data: Value, bin, _| async move {
                    println!("getVersion: {:?} {:?}", data, bin);
                    socket
                        .bin(bin)
                        .emit("receiveVersion", APP_VERSION.to_string())
                        .ok();
                });

                socket.on("settingUpdate", |_, data: Settings, bin, _| async move {
                    println!("settingUpdate: {:?} {:?}", data, bin);
                });

                let rpc_client_manager_clone = Arc::clone(&rpc_client_manager);
                socket.on("setActivity", move |_, data: Presence, bin, _| {
                    println!("setActivity: {:?} {:?}", &data, bin);

                    let client_id: u64 = data.client_id.parse().unwrap();

                    let rpc_client_manager_clone = Arc::clone(&rpc_client_manager_clone);

                    async move {
                        let mut rpc_client_manager = rpc_client_manager_clone.lock().unwrap();
                        if let Some(rpc_client) = rpc_client_manager.get_client_mut(client_id) {
                            rpc_client.set_activity(data);
                        } else {
                            let mut rpc_client = RpcClient::new(client_id);
                            rpc_client.set_activity(data);
                            rpc_client_manager.add_client_instance(rpc_client);
                        }
                    }
                });

                let rpc_client_manager_clone = Arc::clone(&rpc_client_manager);
                socket.on("clearActivity", move |_, data: Value, bin, _| {
                    println!("clearActivity: {:?} {:?}", data, bin);

                    let rpc_client_manager_clone = Arc::clone(&rpc_client_manager_clone);

                    async move {
                        for rpc_client in rpc_client_manager_clone.lock().unwrap().get_clients_mut()
                        {
                            rpc_client.clear_activity();
                        }
                    }
                });

                // TODO: spawn in separate thread for performance?
                socket
                    .emit("discordUser", RpcClient::default().get_user())
                    .ok();
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
