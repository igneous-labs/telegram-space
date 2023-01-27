use log::{debug, trace};
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
    time::Instant,
};

use super::sender_service;
use crate::{
    consts::WORLD_STATE_SYNC_INTERVAL,
    protocol::{ClientId, PlayerStateData},
};

pub struct StateService {
    thread_hdl: JoinHandle<()>,
}

#[derive(Debug)]
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
        let last_synced_at: Option<Instant> = None;
        let last_updated_at: Option<Instant> = None;

        Self {
            thread_hdl: Self::spawn_service(
                world_state,
                last_synced_at,
                last_updated_at,
                message_rx,
                sender_service_tx,
            ),
        }
    }

    fn spawn_service(
        mut world_state: HashMap<ClientId, PlayerStateData>,
        mut last_synced_at: Option<Instant>,
        mut last_updated_at: Option<Instant>,
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> JoinHandle<()> {
        thread::spawn(move || loop {
            if let Ok(msg) = message_rx.try_recv() {
                trace!("Received {:?}", msg);
                match msg {
                    Message::UpdatePlayerState(client_id, player_state_data) => {
                        if !world_state.contains_key(&client_id)
                            || world_state.get(&client_id).unwrap() != &player_state_data
                        {
                            debug!("Updating world state for client #{}", client_id);
                            world_state.insert(client_id, player_state_data);
                            last_updated_at = Some(Instant::now());
                        }
                    }
                    Message::RemovePlayerState(client_id) => {
                        debug!("Removing client #{} from world state", client_id);
                        world_state.remove(&client_id);
                        last_updated_at = Some(Instant::now());
                    }
                }
            }
            if last_synced_at.is_none()
                || (last_updated_at.is_some() && last_updated_at.unwrap() > last_synced_at.unwrap())
                    && Instant::now().duration_since(last_synced_at.unwrap())
                        > WORLD_STATE_SYNC_INTERVAL
            {
                trace!("Sending world state to SenderService");
                sender_service_tx
                    .send(sender_service::Message::SyncWorldState(world_state.clone()))
                    .expect("failed to send to SenderService");
                last_synced_at = Some(Instant::now());
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
