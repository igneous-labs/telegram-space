use log::{debug, trace, warn};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
    collections::HashMap,
};

use super::sender_service;
use crate::protocol::{ClientId, InstanceId, LevelId, PlayerStateData};
use world_state::WorldState;

mod world_state;

pub struct StateService {
    thread_hdl: JoinHandle<()>,
}

#[derive(Debug)]
pub enum Message {
    UpdatePlayerState(ClientId, PlayerStateData),
    RemovePlayerState(ClientId),
    UpdatePlayerInstance(ClientId, InstanceId),
    CreateInstance(InstanceId, LevelId),
    UpdatePlayerChatId(ClientId, Vec<u8>),
}

impl StateService {
    pub fn new(
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> Self {
        let world_state = WorldState::new();
        let client_chat_id = HashMap::new();

        Self {
            thread_hdl: Self::spawn_service(world_state, client_chat_id, message_rx, sender_service_tx),
        }
    }

    fn spawn_service(
        mut world_state: WorldState,
        mut client_chat_id: HashMap<ClientId, Vec<u8>>,
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> JoinHandle<()> {
        thread::spawn(move || loop {
            if let Ok(msg) = message_rx.try_recv() {
                trace!("Received {:?}", msg);
                match msg {
                    Message::UpdatePlayerState(client_id, player_state_data) => {
                        if !world_state.has_client(&client_id) {
                            warn!(
                                "Client #{} is not registered to any instance, ignoring",
                                client_id
                            );
                        } else if !world_state.has_player_state(&client_id)
                            || world_state.get_player_state_data(&client_id) != &player_state_data
                        {
                            // upsert case
                            debug!("Updating player state for client #{}", client_id);
                            world_state.update_player_state(&client_id, player_state_data);
                        }
                    }
                    Message::RemovePlayerState(client_id) => {
                        debug!("Removing client #{} from world state", client_id);
                        if world_state.has_client(&client_id) {
                            world_state.remove_player_state(&client_id);
                        }
                        debug!("Removing client #{} from client_chat_id map", client_id);
                        if client_chat_id.contains_key(&client_id) {
                            client_chat_id.remove(&client_id);
                        }
                    }
                    Message::UpdatePlayerInstance(client_id, instance_id) => {
                        debug!(
                            "Updating the instance of client #{} to instance #{}",
                            client_id, instance_id
                        );
                        if !world_state.has_instance(&instance_id) {
                            warn!("Could not find instance #{}, ignoring", instance_id);
                        } else {
                            if !world_state.has_client(&client_id) {
                                debug!("Adding client #{} to instance #{}", client_id, instance_id);
                                // Add case
                                world_state.add_player_to_instance(&client_id, &instance_id)
                            } else {
                                debug!("Moving client #{} to instance #{}", client_id, instance_id);
                                // Move case
                                world_state.move_player_to_instance(&instance_id, &client_id);
                            }
                            sender_service_tx
                                .send(sender_service::Message::PlayerInstanceAcknowledge(
                                    client_id,
                                    world_state.get_instance_level_id(&instance_id).to_owned(),
                                ))
                                .unwrap_or_else(|err| {
                                    warn!("failed to send to SenderService: {}", err)
                                });
                        }
                    }
                    Message::CreateInstance(instance_id, level_id) => {
                        if world_state.has_instance(&instance_id) {
                            warn!(
                                "World state already has instance #{}, ignoring",
                                instance_id
                            );
                        } else {
                            debug!(
                                "Creating instance #{} with level #{}",
                                instance_id, level_id
                            );
                            world_state.add_instance(&instance_id, &level_id);
                        }
                    }
                    Message::UpdatePlayerChatId(client_id, chat_id) => {
                        debug!("Updating client #{}'s chat id to '{}'", client_id, String::from_utf8(chat_id.clone()).unwrap());
                        // TODO: think about invariants
                        //  - client_id exists in world_state?
                        //  - client_id is not part of an instance?
                        // For now just populate the client_chat_id map
                        client_chat_id.insert(client_id, chat_id);
                    }
                }
            }

            let instance_ids = world_state.get_instance_ids_to_sync();
            if !instance_ids.is_empty() {
                debug!(
                    "Sending world instances #{:?} states to SenderService",
                    instance_ids
                );
                for instance_id in &instance_ids {
                    let instance_state = world_state.get_instance_state(&instance_id);
                    if !instance_state.is_empty() {
                        sender_service_tx
                            .send(sender_service::Message::SyncWorldState(
                                instance_state.clone(),
                            ))
                            .unwrap_or_else(|err| {
                                warn!("failed to send to SenderService: {}", err)
                            });
                    }
                }
                world_state.update_last_synced_at(&instance_ids);
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
