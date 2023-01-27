use log::{info, warn};
use simple_websockets::{Event, EventHub, Message};
use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use super::{sender_service, state_service};
use crate::protocol::{ClientId, IngressMessage};

const PORT: u16 = 1337;

pub struct ReceiverService {
    thread_hdl: JoinHandle<()>,
}

impl ReceiverService {
    pub fn new(
        state_service_tx: Sender<state_service::Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> Self {
        let event_hub = simple_websockets::launch(PORT).expect("Failed to listen");

        Self {
            thread_hdl: Self::spawn_service(state_service_tx, sender_service_tx, event_hub),
        }
    }

    fn spawn_service(
        state_service_tx: Sender<state_service::Message>,
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
                        .expect("failed to send to SenderService");
                }
                Event::Disconnect(client_id) => {
                    let client_id =
                        ClientId::try_from(client_id).expect("max number of connection exceeded");
                    info!("Client #{} disconnected.", client_id);
                    sender_service_tx
                        .send(sender_service::Message::Unregister(client_id))
                        .expect("failed to send to SenderService");
                    state_service_tx
                        .send(state_service::Message::RemovePlayerState(client_id))
                        .expect("failed to send to StateService");
                }
                Event::Message(client_id, Message::Binary(data)) => {
                    let client_id =
                        ClientId::try_from(client_id).expect("max number of connection exceeded");
                    let msg = IngressMessage::try_from(data.as_slice()).unwrap();
                    match msg {
                        IngressMessage::PlayerState(data) => {
                            state_service_tx
                                .send(state_service::Message::UpdatePlayerState(client_id, data))
                                .expect("failed to send to StateService");
                        }
                    }
                }
                unidentified_message => {
                    warn!("received unidentified message: {:?}", unidentified_message);
                }
            }
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}
