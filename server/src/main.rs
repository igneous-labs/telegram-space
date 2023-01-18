use simple_websockets::{Event, Responder};
use std::collections::HashMap;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

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
                info!("Client #{} connected", client_id);
                // add their Responder to our `clients` map:
                clients.insert(client_id, responder);
            },
            Event::Disconnect(client_id) => {
                info!("Client #{} disconnected", client_id);
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
