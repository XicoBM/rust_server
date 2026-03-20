use axum::{
    extract::Json,
    http::StatusCode,
    routing::get,
    Router,
};
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use serde::Deserialize;
use std::env;
use dotenvy::dotenv;
use local_ip_address::local_ip;

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

    let text = format!(
        "Device: {}\n Message: {}",
        payload.device_id, payload.message
    );

    if let Err(e) = send_email_alert(&text).await {
        eprintln!("Error sending message to teams: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

// Function responsible for the sending of emails
async fn send_email_alert(body: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let body = body.to_string();

    let result = tokio::task::spawn_blocking(move || {
        let credentials_one = env::var("EMAIL_USER").expect("EMAIL_USER not set");
        let credentials_two = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not correct");
        let credentials_three =env::var("EMAIL_RECEIVER").expect("EMAIL_RECEIVER not set up");

        let email = Message::builder()
            .from(Mailbox::new(
                Some("Servidor Rust".into()),
                credentials_one.parse()?,
            ))
            .to(credentials_three.parse()?)
            .subject("Alerta do sistema")
            .body(body)?;

        let creds = Credentials::new(credentials_one.to_string(), credentials_two.to_string());

        let tls_parameters = TlsParameters::new("smtp.gmail.com".to_string())?;

        let mailer = SmtpTransport::relay("smtp.gmail.com")?
            .port(587)
            .tls(Tls::Required(tls_parameters))
            .credentials(creds)
            .build();

        mailer.send(&email)?;
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await?;

    result?;

    Ok(())
}

// This function serves as a simple way for testing/simulating the ESP32 signal
async fn signal_testing() -> Result<StatusCode, StatusCode> {
    let device = "huawei_pc";
    let message = "testing again";

    let text = format!("Device: {}\n Message: {}", device, message);

    if let Err(e) = send_email_alert(&text).await {
        eprintln!("Error sending message to teams: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/", get(|| async { "Testing" }))
        .route("/signal", get(signal_testing).post(receive_signal));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:12345").await.unwrap();

    match local_ip() {
        Ok(ip) => println!("Server running on {:?}:12345", ip),
        Err(_e) => eprintln!("Error obtaining the local IP address.")
    }

    axum::serve(listener, app).await.unwrap();
}


/**
 * Main problems with this code:
 *  1. Both routes defined are unprotected and/or exposed:
 *      1.1  In the /signal endpoint there is no authentication, so it is possible to uncover the IP and port in use;
 *           This could be solved by implementing a static token in the header of every request.
 * 
 *      1.2 In the same endpoint, the get is also exposed without authentication.
 * 
 *  2. The payload is not limited. Since Axum accepts, by defect, big bodies, it is possible to send a JSON to big for the program.
 * 
 *  3. The data in the payload that is received is sent to the email without treatment, so special characters are going to be passed along.      
 */