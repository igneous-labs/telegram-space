use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
};

use super::sender_service;
use crate::protocol::{ClientId, PlayerStateData};

pub struct StateService {
    thread_hdl: JoinHandle<()>,
}

pub enum Message {
    UpdatePlayerState(ClientId, PlayerStateData),
    RemovePlayerState(ClientId),
}

impl StateService {
    pub fn new(
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> Self {
        let world_state: HashMap<ClientId, PlayerStateData> = HashMap::new();

        Self {
            thread_hdl: Self::spawn_service(world_state, message_rx, sender_service_tx),
        }
    }

    fn spawn_service(
        mut world_state: HashMap<ClientId, PlayerStateData>,
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            for msg in message_rx.iter() {
                //info!("received {:?}", player_state);
                match msg {
                    Message::UpdatePlayerState(client_id, player_state_data) => {
                        world_state.insert(client_id, player_state_data);
                        sender_service_tx
                            .send(sender_service::Message::WorldState(world_state.clone()))
                            .expect("failed to send to SenderService");
                    }
                    Message::RemovePlayerState(client_id) => {
                        world_state.remove(&client_id);
                    }
                }
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
