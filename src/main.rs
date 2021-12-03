use std::time::Duration;

use tinyroute::client::{connect, TcpClient, ClientMessage};
use tinyroute::frame::Frame;


async fn run() {
    // Try to connect to the NeoTwitch instance
    let connection = match TcpClient::connect("127.0.0.1:6000").await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to connect: {}", e);
            return;
        }
    };

    let (client_tx, client_rx) = connect(connection, Some(Duration::from_secs(60)));

    // Subscribe to chat and channel events
    let msg = b"chat|sub";
    let framed_message = Frame::frame_message(msg);
    client_tx.send_async(ClientMessage::Payload(framed_message)).await.unwrap();

    let msg = b"chat|cpoints";
    let framed_message = Frame::frame_message(msg);
    client_tx.send_async(ClientMessage::Payload(framed_message)).await.unwrap();

    while let Ok(msg) = client_rx.recv_async().await {
        // Deserialize message
        // do something with it
        eprintln!("{}", String::from_utf8(msg).unwrap());
    }

}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    run().await;
}
