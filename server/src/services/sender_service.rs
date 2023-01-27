use simple_websockets::{Message as WebsocketMessage, Responder};
use std::{
    collections::HashMap,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use crate::protocol::{
    ClientId, EgressMessage, EgressMessageType, PlayerStateData, WorldStateEntry,
};

pub struct SenderService {
    thread_hdl: JoinHandle<()>,
}

#[derive(Debug)]
pub enum Message {
    Register(ClientId, Responder),                  // add a new client
    Unregister(ClientId),                           // remove a disconnected client
    WorldState(HashMap<ClientId, PlayerStateData>), // broadcast the current world state
}

impl SenderService {
    pub fn new(message_rx: Receiver<Message>) -> Self {
        let clients: HashMap<ClientId, Responder> = HashMap::new();

        Self {
            thread_hdl: Self::spawn_service(clients, message_rx),
        }
    }

    fn spawn_service(
        mut clients: HashMap<ClientId, Responder>,
        message_rx: Receiver<Message>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            for msg in message_rx.iter() {
                match msg {
                    Message::WorldState(world_state) => {
                        for (dest_client_id, responder) in clients.iter() {
                            let world_state_msg = EgressMessage::WorldState(
                                world_state
                                    .iter()
                                    .filter_map(|(&client_id, &player_state_data)| {
                                        if *dest_client_id != client_id {
                                            Some(
                                                (WorldStateEntry {
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
                                let payload = WebsocketMessage::Binary((&world_state_msg).into());
                                responder.send(payload);
                            }
                        }
                    }
                    Message::Register(client_id, responder) => {
                        let payload = WebsocketMessage::Binary(
                            [
                                u8::from(EgressMessageType::Acknowledge)
                                    .to_le_bytes()
                                    .to_vec(),
                                client_id.to_le_bytes().to_vec(),
                            ]
                            .concat(),
                        );
                        responder.send(payload);
                        clients.insert(client_id, responder);
                    }
                    Message::Unregister(client_id) => {
                        clients.remove(&client_id);
                    }
                }
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
