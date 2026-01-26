use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

// Expected payload, sent by an ESP32
#[derive(Deserialize, Debug)]
struct Signal {
    device_id: String,
    message: String,
}

// HTTP handler, responsible for receiving information in the form of "Signal"
async fn receive_signal(Json(payload): Json<Signal>) -> Result<StatusCode, StatusCode> {
    println!(
        "Received signal from {}: {}",
        payload.device_id, payload.message
    );

    let text = format!("Device: {}\n Message: {}", payload.device_id, payload.message); 

    let Err(e) = send_teams_message(&text).await {
        eprintln!("Error sending message to teams: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::Ok)
}

// Sends message to a Microsoft Teams page through an incoming webhook
async fn send_teams_message() {}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello World" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
