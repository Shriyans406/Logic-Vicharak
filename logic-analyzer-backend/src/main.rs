use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // 1. Initialize Logging (to see what's happening in the terminal)
    tracing_subscriber::fmt::init();

    // 2. Create a Broadcast Channel
    // This allows the Serial Task to send data ONCE, and multiple WebSockets can "subscribe" to it.
    // We set a capacity of 16 messages.
    let (tx, _rx) = broadcast::channel::<Vec<u8>>(16);

    // 3. SPAWN THE MOCK SERIAL TASK
    // Since the board isn't connected yet, we simulate the FPGA/RP2040 sending data.
    let tx_clone = tx.clone();
tokio::spawn(async move {
    // Open the REAL COM port of your Shrike Lite [cite: 2026-01-19, 2026-02-19]
    let mut port = tokio_serial::new("COM3", 921_600) // CHANGE TO YOUR PORT
        .open_native_async()
        .expect("Failed to open hardware port");

    let mut serial_buf = vec![0; 1024];
    loop {
        if let Ok(n) = port.read(&mut serial_buf).await {
            if n > 0 {
                // Pipe the hardware bytes directly to the WebSockets [cite: 2026-02-19]
                let _ = tx_clone.send(serial_buf[..n].to_vec());
            }
        }
    }
});

    // 4. DEFINE THE ROUTER
    let app = Router::new()
        .route("/ws", get(move |ws| ws_handler(ws, tx.clone())))
        .layer(CorsLayer::permissive()); // Allow the Next.js frontend to connect

    // 5. START THE SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Backend Server running on http://{}", addr);
    println!("WebSocket endpoint: ws://{}/ws", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 6. THE WEBSOCKET HANDLER
async fn ws_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<Vec<u8>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<Vec<u8>>) {
    // Subscribe to the broadcast channel
    let mut rx = tx.subscribe();

    println!("Client connected to WebSocket");

    // Loop: Whenever the Broadcast Channel gets data, push it to this WebSocket
    while let Ok(msg_content) = rx.recv().await {
        if socket.send(Message::Binary(msg_content)).await.is_err() {
            // If the client disconnects, break the loop
            println!("Client disconnected");
            break;
        }
    }
}