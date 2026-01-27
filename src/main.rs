use axum::{
    extract::Json,
    http::StatusCode,
    routing::{post},
    Router,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::{env, net::SocketAddr};

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

    if let Err(e) = send_teams_message(&text).await {
        eprintln!("Error sending message to teams: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

// Sends message to a Microsoft Teams page through an incoming webhook
async fn send_teams_message(text: &str) -> Result<(), reqwest::Error> {
    let webhook_url = env::var("TEAMS_WEBHOOK_URL").expect("TEAMS_WEBHOOK_URL not set up");

    let payload = json!({
        "@type": "MessageCard",
        "@context": "http://schema.org/extensions",
        "summary": "New Event",
        "themeColor": "0076D7",
        "title": "Event Received",
        "text": text
    });

    Client::new().post(webhook_url).json(&payload).send().await?.error_for_status()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    send_teams_message("Teste direto do Rust 🦀")
        .await
        .unwrap();
}
