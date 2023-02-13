use log::{debug, info, trace, warn};
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
};

use super::sender_service;
use crate::{
    consts::LEVEL_DATA_DIR,
    protocol::{ClientId, LevelId},
};

pub struct LevelService {
    thread_hdl: JoinHandle<()>,
}

#[derive(Debug)]
pub enum Message {
    UpdateLevel(LevelId, Vec<u8>),
    SendLevel(ClientId, LevelId),
}

impl LevelService {
    pub fn new(
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> Self {
        let levels: HashMap<LevelId, Vec<u8>> = Self::load_levels();

        Self {
            thread_hdl: Self::spawn_service(levels, message_rx, sender_service_tx),
        }
    }

    // TODO: This function should load compressed level from data storage (TBD).
    //       For now it just reads all files from LEVEL_DATA_DIR
    fn load_levels() -> HashMap<LevelId, Vec<u8>> {
        let mut levels: HashMap<LevelId, Vec<u8>> = HashMap::new();

        use std::io::Read;
        for dir_entry in std::fs::read_dir(LEVEL_DATA_DIR).unwrap() {
            if let Ok(dir_entry) = dir_entry {
                let mut f = std::fs::File::open(&dir_entry.path()).expect("no file found");
                let mut buffer = vec![
                    0;
                    dir_entry
                        .metadata()
                        .expect(&format!("could not read metadata of {:?}", dir_entry))
                        .len() as usize
                ];
                f.read(&mut buffer).expect("buffer overflow");
                let level_id: u64 = dir_entry
                    .path()
                    .file_stem()
                    .expect(&format!("could not parse file stem of {:?}", dir_entry))
                    .to_str()
                    .expect(&format!(
                        "could not parse str from file stem of {:?}",
                        dir_entry
                    ))
                    .parse()
                    .expect(&format!(
                        "could not parse u64 from file stem of {:?}",
                        dir_entry
                    ));
                levels.insert(level_id, buffer.clone());

                debug!("Loaded level {}", level_id);
            }
        }
        info!("Loaded {} level(s)", levels.len());

        levels
    }

    fn spawn_service(
        mut levels: HashMap<LevelId, Vec<u8>>,
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            for msg in message_rx.iter() {
                trace!("Received {:?}", msg);
                match msg {
                    Message::UpdateLevel(level_id, level_data) => {
                        // TODO: save level data to the data storage
                        // TODO: sync with clients?
                    }
                    Message::SendLevel(client_id, level_id) => {
                        trace!("Sending level #{} to SenderService", level_id);
                        if let Some(level_data) = levels.get(&level_id) {
                            sender_service_tx
                                .send(sender_service::Message::SendLevel(
                                    client_id,
                                    level_id,
                                    level_data.clone(),
                                ))
                                .expect("failed to send to SenderService");
                        } else {
                            warn!("Could not find level #{}, ignoring request", level_id);
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
