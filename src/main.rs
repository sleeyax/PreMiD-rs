use axum::routing::get;
use axum::Server;
use serde_json::Value;
use socketioxide::{Namespace, SocketIoLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::constants::{APP_VERSION, REPO_URL};

mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let addr = "127.0.0.1:3020";

    info!("Starting server on http://{}", addr);

    let ns = Namespace::builder()
        .add("/", |socket| async move {
            println!("Socket connected with id: {}", socket.sid);

            socket.on("getVersion", |socket, data: Value, bin, _| async move {
                println!("getVersion: {:?} {:?}", data, bin);
                socket.bin(bin).emit("receiveVersion", APP_VERSION.to_string()).ok();
            });

            socket.on("settingUpdate", |_, data: Value, bin, _| async move {
                println!("settingUpdate: {:?} {:?}", data, bin);
            });

            socket.on("setActivity", |_, data: Value, bin, _| async move {
                println!("setActivity: {:?} {:?}", data, bin);
            });

            socket.on("clearActivity", |_, data: Value, bin, _| async move {
                println!("clearActivity: {:?} {:?}", data, bin);
            });
        })
        .build();

    let app = axum::Router::new()
        .route("/", get(|| async { format!("PreMiD-rs server is running. Go to {} for help and support.", REPO_URL) }))
        .layer(SocketIoLayer::new(ns));

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
