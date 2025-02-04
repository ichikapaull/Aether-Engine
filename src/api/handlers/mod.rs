pub mod galaxy;
pub mod star;
pub mod planet;

use std::sync::Arc;
use warp::ws::{Message, WebSocket};
use tokio::sync::mpsc;
use crate::application::simulation::SimulationEngine;

pub async fn ws_handler(
    ws: warp::ws::Ws,
    simulation: Arc<SimulationEngine>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(ws.on_upgrade(move |socket| handle_ws_connection(socket, simulation)))
}

async fn handle_ws_connection(ws: WebSocket, simulation: Arc<SimulationEngine>) {
    let (ws_sender, mut ws_receiver) = ws.split();
    let (tx, rx) = mpsc::channel(32);

    // Gelen mesajları işle
    tokio::task::spawn(async move {
        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(msg) => {
                    if let Ok(text) = msg.to_str() {
                        handle_ws_message(text, &simulation).await;
                    }
                }
                Err(e) => {
                    eprintln!("websocket error: {}", e);
                    break;
                }
            }
        }
    });

    // Simülasyon güncellemelerini gönder
    tokio::task::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = ws_sender.send(Message::text(message)).await {
                eprintln!("websocket send error: {}", e);
                break;
            }
        }
    });
}

async fn handle_ws_message(message: &str, simulation: &Arc<SimulationEngine>) {
    // WebSocket mesaj işleme mantığı
    // TODO: Implement message handling
} 