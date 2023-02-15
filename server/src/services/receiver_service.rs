use log::{info, trace, warn};
use simple_websockets::{Event, EventHub, Message};
use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use super::{level_service, sender_service, state_service};
use crate::{
    consts::PORT,
    protocol::{ClientId, IngressMessage},
};

pub struct ReceiverService {
    thread_hdl: JoinHandle<()>,
}

impl ReceiverService {
    pub fn new(
        state_service_tx: Sender<state_service::Message>,
        level_service_tx: Sender<level_service::Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> Self {
        let event_hub = simple_websockets::launch(PORT).expect("Failed to listen");

        Self {
            thread_hdl: Self::spawn_service(
                state_service_tx,
                level_service_tx,
                sender_service_tx,
                event_hub,
            ),
        }
    }

    fn spawn_service(
        state_service_tx: Sender<state_service::Message>,
        level_service_tx: Sender<level_service::Message>,
        sender_service_tx: Sender<sender_service::Message>,
        event_hub: EventHub,
    ) -> JoinHandle<()> {
        thread::spawn(move || loop {
            match event_hub.poll_event() {
                Event::Connect(client_id, responder) => {
                    let client_id =
                        ClientId::try_from(client_id).expect("max number of connection exceeded");
                    info!("Client #{} connected, acknowledging.", client_id);
                    sender_service_tx
                        .send(sender_service::Message::Register(
                            client_id,
                            responder.clone(),
                        ))
                        .unwrap_or_else(|err| warn!("failed to send to SenderService: {}", err));
                }
                Event::Disconnect(client_id) => {
                    let client_id =
                        ClientId::try_from(client_id).expect("max number of connection exceeded");
                    info!("Client #{} disconnected.", client_id);
                    sender_service_tx
                        .send(sender_service::Message::Deregister(client_id))
                        .unwrap_or_else(|err| warn!("failed to send to SenderService: {}", err));
                    state_service_tx
                        .send(state_service::Message::RemovePlayerState(client_id))
                        .unwrap_or_else(|err| warn!("failed to send to StateService: {}", err));
                }
                Event::Message(client_id, Message::Binary(data)) => {
                    let client_id =
                        ClientId::try_from(client_id).expect("max number of connection exceeded");
                    let msg = IngressMessage::try_from(data.as_slice()).unwrap();
                    match msg {
                        IngressMessage::PlayerState(data) => {
                            trace!("Received player state from client #{}", client_id);
                            state_service_tx
                                .send(state_service::Message::UpdatePlayerState(client_id, data))
                                .unwrap_or_else(|err| {
                                    warn!("failed to send to StateService: {}", err)
                                });
                        }
                        IngressMessage::RequestLevel(level_id) => {
                            trace!("Client #{} requested level #{}", client_id, level_id);
                            level_service_tx
                                .send(level_service::Message::SendLevel(client_id, level_id))
                                .unwrap_or_else(|err| {
                                    warn!("failed to send to StateService: {}", err)
                                });
                        }
                        IngressMessage::PlayerInstance(instance_id) => {
                            trace!(
                                "Client #{} requested to register to instance #{}",
                                client_id,
                                instance_id
                            );
                            state_service_tx
                                .send(state_service::Message::UpdatePlayerInstance(
                                    client_id,
                                    instance_id,
                                ))
                                .unwrap_or_else(|err| {
                                    warn!("failed to send to StateService: {}", err)
                                });
                        }
                        IngressMessage::PlayerChatId(chat_id) => {
                            state_service_tx
                                .send(state_service::Message::UpdatePlayerChatId(client_id, chat_id))
                                .unwrap_or_else(|err| {
                                    warn!("failed to send to StateService: {}", err)
                                });
                        }
                    }
                }
                unidentified_message => {
                    warn!("Received unidentified message: {:?}", unidentified_message);
                }
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
