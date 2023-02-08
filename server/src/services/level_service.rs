use log::{debug, trace, warn};
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
};

use super::sender_service;
use crate::{
    consts::LEVEL_DATA_DIR,
    protocol::{ClientId, CompressedLevelData, LevelId},
};

pub struct LevelService {
    thread_hdl: JoinHandle<()>,
}

#[derive(Debug)]
pub enum Message {
    UpdateLevel(LevelId, CompressedLevelData),
    SendLevel(ClientId, LevelId),
}

impl LevelService {
    pub fn new(
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> Self {
        let levels: HashMap<LevelId, CompressedLevelData> = Self::load_levels();

        Self {
            thread_hdl: Self::spawn_service(levels, message_rx, sender_service_tx),
        }
    }

    // TODO: This function should load compressed level from data storage (TBD).
    //       For now it just reads test_level from file as level_id = 0.
    fn load_levels() -> HashMap<LevelId, CompressedLevelData> {
        let mut levels = HashMap::new();

        {
            use std::io::Read;
            let data_file = std::path::Path::new(LEVEL_DATA_DIR).join("0.dat");
            let mut f = std::fs::File::open(&data_file).expect("no file found");
            let metadata = std::fs::metadata(&data_file).expect("unable to read metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            f.read(&mut buffer).expect("buffer overflow");
            // make three test levels (identical levels for now)
            levels.insert(
                0,
                CompressedLevelData {
                    decompressed_size: 230996,
                    data: buffer.clone(),
                },
            );
            levels.insert(
                1,
                CompressedLevelData {
                    decompressed_size: 230996,
                    data: buffer.clone(),
                },
            );
            levels.insert(
                2,
                CompressedLevelData {
                    decompressed_size: 230996,
                    data: buffer,
                },
            );
            debug!("Loaded {} level(s)", levels.len());
        }

        levels
    }

    fn spawn_service(
        mut levels: HashMap<LevelId, CompressedLevelData>,
        message_rx: Receiver<Message>,
        sender_service_tx: Sender<sender_service::Message>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            for msg in message_rx.iter() {
                trace!("Received {:?}", msg);
                match msg {
                    Message::UpdateLevel(level_id, compressed_level_data) => {
                        levels.insert(level_id, compressed_level_data);
                        // TODO: save level data to the data storage
                        // TODO: sync with clients?
                    }
                    Message::SendLevel(client_id, level_id) => {
                        trace!("Sending level #{} to SenderService", level_id);
                        if let Some(compressed_level_data) = levels.get(&level_id) {
                            sender_service_tx
                                .send(sender_service::Message::SendLevel(
                                    client_id,
                                    level_id,
                                    compressed_level_data.clone(),
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
