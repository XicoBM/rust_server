use axum::{
    routing::{get, post},
    http::StatusCode,
    extract::Json,
    Router,
};
use tokio::net::TcpListener;
use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use std::{env, net::SocketAddr};

#[derive(Deserialize)]
struct Signal {

}

async fn receive_signal() {}

async fn send_teams_message() {}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {"Hello World"}));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}