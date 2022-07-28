//! A simple echo server.
//!
//! You can test this out by running:
//!
//!     cargo run --example server 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run --example client ws://127.0.0.1:12345/

use dcl_protocol::renderer_protocol::RendererProtocol::CRDTManyMessages;
use protobuf::Message;

use futures_util::SinkExt;
use std::env;

use futures_util::StreamExt;

use tokio::net::{TcpListener, TcpStream};

use tokio::task;

use dcl_common::Result;

pub async fn start() -> Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    task::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
    });

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg.unwrap();
        println!("{:?}", msg);
        if msg.is_binary() {
            let message = CRDTManyMessages::parse_from_bytes(msg.into_data().as_slice());
            println!("{:?}", message);
        }
    }
}
