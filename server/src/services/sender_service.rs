use log::{debug, info, trace, warn};
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
    Register(ClientId, Responder), // add a new client
    Deregister(ClientId),          // remove a disconnected client
    SyncWorldState(HashMap<ClientId, PlayerStateData>, Vec<(ClientId, Vec<u8>)>), // broadcast the current world state
    SendLevel(ClientId, LevelId, Vec<u8>),
    PlayerInstanceAcknowledge(ClientId, LevelId),
    PlayerChatUserIdAcknowledge(ClientId),
}

impl SenderService {
    pub fn new(message_rx: Receiver<Message>) -> Self {
        info!("Initializing SenderService");
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
                    Message::SyncWorldState(world_state, instance_chat_user_ids) => {
                        debug!(
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
                            trace!("sending world state to client #{}", dest_client_id);
                            Self::try_send(
                                dest_client_id,
                                &clients,
                                EgressMessage::WorldState(
                                    world_state_data,
                                    instance_chat_user_ids.clone(),
                                ),
                            )
                            .unwrap_or_else(|err| warn!("{}", err));
                        }
                    }
                    Message::Register(client_id, responder) => {
                        debug!("Registering client #{}", client_id);
                        clients.insert(client_id, responder);
                        Self::try_send(&client_id, &clients, EgressMessage::Acknowledge(client_id))
                            .unwrap_or_else(|err| warn!("{}", err));
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
                        Self::try_send(
                            &client_id,
                            &clients,
                            EgressMessage::LevelData(level_id, level_data),
                        )
                        .unwrap_or_else(|err| warn!("{}", err));
                    }
                    Message::PlayerInstanceAcknowledge(client_id, level_id) => {
                        debug!("Sending level id #{} to client #{}", level_id, client_id);
                        Self::try_send(
                            &client_id,
                            &clients,
                            EgressMessage::PlayerInstanceAcknowledge(level_id),
                        )
                        .unwrap_or_else(|err| warn!("{}", err));
                    }
                    Message::PlayerChatUserIdAcknowledge(client_id) => {
                        debug!("Acknowledging client #{}'s chat_user_id", client_id);
                        Self::try_send(
                            &client_id,
                            &clients,
                            EgressMessage::PlayerChatUserIdAcknowledge,
                        )
                        .unwrap_or_else(|err| warn!("{}", err));
                    }
                }
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }

    #[inline(always)]
    fn try_send(
        client_id: &ClientId,
        clients: &HashMap<ClientId, Responder>,
        message: EgressMessage,
    ) -> Result<(), String> {
        if let Some(responder) = clients.get(client_id) {
            if responder.send(WebsocketMessage::Binary((&message).into())) {
                Ok(())
            } else {
                Err(format!("Client #{} disconnected, ignoring request", client_id).into())
            }
        } else {
            Err(format!("Could not find client #{}, ignoring request", client_id).into())
        }
    }
}
