use log::{info, warn, LevelFilter};
use simple_logger::SimpleLogger;
use simple_websockets::{Event, Message, Responder};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod protocol;
//mod state;

use protocol::{ClientId, EgressMessage, EgressMessageType, IngressMessage, PlayerStateData};

const PORT: u16 = 1337;
const WORLD_STATE_BROADCAST_INTERVAL_MS: u64 = 20;

// NOTE: client_id is downcasted from u64 to u16, and the implementation assumes that
//       the max connection is kept at u16 max.
fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("telegram_space_server", LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();

    let event_hub = simple_websockets::launch(PORT).expect("Failed to listen");

    // TODO: do message passing using state::StateService instead of resource sharing
    let world_state: Arc<Mutex<HashMap<ClientId, PlayerStateData>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // map between client ids and the client's `Responder`:
    let clients: Arc<Mutex<HashMap<ClientId, Responder>>> = Arc::new(Mutex::new(HashMap::new()));

    // broad cast world state
    let sender_handle = {
        let clients = clients.clone();
        let world_state = world_state.clone();
        thread::spawn(move || loop {
            {
                let clients = clients.lock().unwrap();
                let world_state = world_state.lock().unwrap();
                for (dest_client_id, responder) in clients.iter() {
                    //info!("sending world state to client_id: {}", client_id);
                    let world_state_msg = EgressMessage::WorldState(
                        world_state
                            .iter()
                            .filter_map(|(&client_id, &player_state_data)| {
                                if *dest_client_id != client_id {
                                    Some(
                                        (protocol::WorldStateEntry {
                                            client_id,
                                            player_state_data,
                                        })
                                        .into(),
                                    )
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    );
                    if world_state.len() != 0 {
                        let payload = Message::Binary((&world_state_msg).into());
                        responder.send(payload);
                    }
                }
            }
            thread::sleep(Duration::from_millis(WORLD_STATE_BROADCAST_INTERVAL_MS));
        })
    };

    info!("Websocket echo server initialized");
    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                let client_id =
                    ClientId::try_from(client_id).expect("max number of connection exceeded");
                info!("Client #{} connected, acknowledging.", client_id);
                // add their Responder to our `clients` map:
                let mut clients = clients.lock().unwrap();
                clients.insert(client_id, responder.clone());

                info!("# of clients: {}", clients.len());

                let payload = Message::Binary(
                    [
                        u8::from(EgressMessageType::Acknowledge)
                            .to_le_bytes()
                            .to_vec(),
                        client_id.to_le_bytes().to_vec(),
                    ]
                    .concat(),
                );
                responder.send(payload);
            }
            Event::Disconnect(client_id) => {
                let client_id =
                    ClientId::try_from(client_id).expect("max number of connection exceeded");
                info!("Client #{} disconnected.", client_id);
                // remove the disconnected client from the clients map:
                let mut clients = clients.lock().unwrap();
                clients.remove(&client_id);

                info!("# of clients: {}", clients.len());

                let mut world_state = world_state.lock().unwrap();
                world_state.remove(&client_id);
            }
            Event::Message(client_id, Message::Binary(data)) => {
                let client_id =
                    ClientId::try_from(client_id).expect("max number of connection exceeded");
                let msg = IngressMessage::try_from(data.as_slice()).unwrap();
                //info!("Client #{} messaged: {:?}", client_id, msg);

                // TODO: move this to StateService
                if let IngressMessage::PlayerState(data) = msg {
                    let mut world_state = world_state.lock().unwrap();
                    world_state.insert(client_id, data);
                }
            }
            unidentified_message => {
                warn!("received unidentified message: {:?}", unidentified_message);
            }
        }
    }

    sender_handle.join().unwrap();
}
