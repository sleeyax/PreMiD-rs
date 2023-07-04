use axum::routing::get;
use axum::Server;
use serde_json::Value;
use socketioxide::{Namespace, SocketIoLayer};
use std::sync::{Arc, Mutex};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{
    constants::{APP_VERSION, DISCORD_APP_ID, REPO_URL},
    settings::Settings,
    types::Presence,
};

mod constants;
mod rpc_client;
mod settings;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let rpc_client = Arc::new(Mutex::new(rpc_client::RpcClient::new(DISCORD_APP_ID)));

    let addr = "127.0.0.1:3020";

    info!("Starting server on http://{}", addr);

    let rpc_client_clone = Arc::clone(&rpc_client);
    let ns = Namespace::builder()
        .add("/", move |socket| {
            let rpc_client = Arc::clone(&rpc_client_clone);

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

                let rpc_client_clone = Arc::clone(&rpc_client);
                socket.on("setActivity", move |_, data: Presence, bin, _| {
                    let rpc_client_clone = Arc::clone(&rpc_client_clone);

                    async move {
                        println!("setActivity: {:?} {:?}", &data, bin);
                        rpc_client_clone.lock().unwrap().set_activity(data);
                    }
                });

                socket.on("clearActivity", |_, data: Value, bin, _| async move {
                    println!("clearActivity: {:?} {:?}", data, bin);
                });

                let rpc_client_clone = Arc::clone(&rpc_client);
                socket
                    .emit("discordUser", rpc_client_clone.lock().unwrap().get_user())
                    .ok();
            }
        })
        .build();

    // TODO: fix this?
    /* let rpc_client_clone = Arc::clone(&rpc_client);
    ctrlc::set_handler(move || {
        println!("[RPC Client] Exiting");
        rpc_client_clone.lock().unwrap().clear_activity();
        std::process::exit(0);
    })
    .unwrap(); */

    /*  let rpc_client_clone = Arc::clone(&rpc_client);
    rpc_client_clone.lock().unwrap().join_thread(); */
    // --

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

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
