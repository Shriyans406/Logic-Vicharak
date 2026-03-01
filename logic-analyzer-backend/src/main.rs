use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tokio::io::AsyncReadExt;

// These are the specific traits needed for tokio-serial 0.5 [cite: 2026-02-14, 2026-02-19]
//use tokio_serial::{SerialPortBuilderExt};
use tokio_serial::SerialStream;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 1. Create the Broadcast Channel [cite: 2026-02-14]
    let (tx, _rx) = broadcast::channel::<Vec<u8>>(16);

    // 2. The Fixed Serial Task [cite: 2026-02-14, 2026-02-19]
    let tx_clone = tx.clone();
tokio::spawn(async move {
    let builder = tokio_serial::new("COM9", 921_600);

    let mut port = SerialStream::open(&builder)
        .expect("Failed to open Shrike Lite hardware port");

    println!("Successfully hooked into Shrike Lite on COM9");

    let mut buffer = [0u8; 1024];

    loop {
        match port.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let _ = tx_clone.send(buffer[..n].to_vec());
            }
            _ => {}
        }
    }
});

    // 3. Define the Web Server [cite: 2026-02-14, 2026-02-19]
    let app = Router::new()
        .route("/ws", get(move |ws| ws_handler(ws, tx.clone())))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Logic Analyzer Backend online at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<Vec<u8>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<Vec<u8>>) {
    let mut rx = tx.subscribe();
    println!("Web Dashboard connected to stream");

    while let Ok(msg_content) = rx.recv().await {
        if socket.send(Message::Binary(msg_content)).await.is_err() {
            println!("Web Dashboard disconnected");
            break;
        }
    }
}