use log::{debug, trace, warn};
use simple_websockets::{Message as WebsocketMessage, Responder};
use std::{
    collections::HashMap,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use crate::protocol::{ClientId, EgressMessage, LevelId, PlayerStateData, WorldStateEntry};

pub struct SenderService {
    thread_hdl: JoinHandle<()>,
}

#[derive(Debug)]
pub enum Message {
    Register(ClientId, Responder),                      // add a new client
    Deregister(ClientId),                               // remove a disconnected client
    SyncWorldState(HashMap<ClientId, PlayerStateData>), // broadcast the current world state
    SendLevel(ClientId, LevelId, Vec<u8>),
    PlayerInstanceAcknowledge(ClientId, LevelId),
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
                trace!("Received {:?}", msg);
                match msg {
                    Message::SyncWorldState(world_state) => {
                        trace!(
                            "Broadcasting world state to clients: {:?}",
                            world_state.keys()
                        );
                        for dest_client_id in world_state.keys() {
                            let world_state_data: Vec<_> = world_state
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
                                .collect();
                            debug!("sending world state to client #{}", dest_client_id);
                            let payload = WebsocketMessage::Binary(
                                (&EgressMessage::WorldState(world_state_data)).into(),
                            );
                            let responder = clients.get(dest_client_id).unwrap();
                            responder.send(payload);
                        }
                    }
                    Message::Register(client_id, responder) => {
                        debug!("Registering client #{}", client_id);
                        responder.send(WebsocketMessage::Binary(
                            (&EgressMessage::Acknowledge(client_id)).into(),
                        ));
                        clients.insert(client_id, responder);
                    }
                    Message::Deregister(client_id) => {
                        debug!("Deregistering client #{}", client_id);
                        clients.remove(&client_id);
                    }
                    Message::SendLevel(client_id, level_id, level_data) => {
                        debug!(
                            "Sending level data for level #{} to client #{}",
                            level_id, client_id
                        );
                        if let Some(responder) = clients.get(&client_id) {
                            responder.send(WebsocketMessage::Binary(
                                (&EgressMessage::LevelData(level_id, level_data)).into(),
                            ));
                        } else {
                            warn!("Could not find client #{}, ignoring request", client_id);
                        }
                    }
                    Message::PlayerInstanceAcknowledge(client_id, level_id) => {
                        debug!("Sending level id #{} to client #{}", level_id, client_id);
                        if let Some(responder) = clients.get(&client_id) {
                            responder.send(WebsocketMessage::Binary(
                                (&EgressMessage::PlayerInstanceAcknowledge(level_id)).into(),
                            ));
                        } else {
                            warn!("Could not find client #{}, ignoring request", client_id);
                        }
                    }
                }
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
