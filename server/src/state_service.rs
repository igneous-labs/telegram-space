// TODO
use log::info;

use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

pub struct StateService {
    thread_hdl: JoinHandle<()>,
}

impl StateService {
    pub fn new(player_state_receiver: Receiver<&[u8]>) -> Self {
        let thread_hdl = thread::spawn(move || {
            for player_state in player_state_receiver.iter() {
                info!("received {:?}", player_state);
            }
        });
        Self { thread_hdl }
    }
}
