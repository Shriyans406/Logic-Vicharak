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
        let mut counter: u8 = 0;
        loop {
            // Simulate 8-bit logic signals (a walking bit pattern)
            counter = counter.wrapping_add(1);
            let mock_data = vec![counter, counter.reverse_bits(), 0b10101010]; 
            
            // Send to the broadcast channel
            let _ = tx_clone.send(mock_data);
            
            // Simulate high-frequency sampling (every 10ms for now)
            tokio::time::sleep(Duration::from_millis(10)).await;
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