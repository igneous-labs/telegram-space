use simple_websockets::{Event, Responder, Message};
use std::collections::HashMap;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

mod protocol;
use protocol::MessageType;

const PORT: u16 = 1337;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("telegram_space_server", LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();

    let event_hub = simple_websockets::launch(PORT)
        .expect("Failed to listen");

    // map between client ids and the client's `Responder`:
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    info!("Websocket echo server initialized");
    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                info!("Client #{} connected, acknowledging.", client_id);
                // add their Responder to our `clients` map:
                clients.insert(client_id, responder.clone());

                let message_type: u8 = MessageType::Acknowledge.into();
                let payload = Message::Binary([
                    message_type.to_le_bytes().to_vec(),
                    client_id.to_le_bytes().to_vec()
                ].concat());
                responder.send(payload);
            },
            Event::Disconnect(client_id) => {
                info!("Client #{} disconnected.", client_id);
                // remove the disconnected client from the clients map:
                clients.remove(&client_id);
            },
            Event::Message(client_id, message) => {
                info!("Client #{} messaged: {:?}", client_id, message);
                // retrieve this client's `Responder`:
                let responder = clients.get(&client_id).unwrap();
                // echo the message back:
                responder.send(message);
            },
        }
    }
}
