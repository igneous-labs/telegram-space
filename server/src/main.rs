use dotenv::dotenv;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use std::sync::mpsc::channel;

mod envs;
mod protocol;
mod services;

// NOTE: client_id is downcasted from u64 to u16, and the implementation assumes that
//       the max connection is kept at u16 max.
fn main() {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("telegram_space_server", LevelFilter::Debug)
        .with_utc_timestamps()
        .init()
        .unwrap();

    let (state_service_tx, state_service_rx) = channel();
    let (level_service_tx, level_service_rx) = channel();
    let (sender_service_tx, sender_service_rx) = channel();
    let state_service = services::StateService::new(state_service_rx, sender_service_tx.clone());
    let level_service = services::LevelService::new(level_service_rx, sender_service_tx.clone());
    let sender_service = services::SenderService::new(sender_service_rx);
    let receiver_service =
        // TODO: remove clone from state_service_tx
        services::ReceiverService::new(state_service_tx.clone(), level_service_tx, sender_service_tx);
    // DELETEME: temporarily setup testing instances
    state_service_tx
        .send(services::state_service::Message::CreateInstance(0, 0))
        .expect("failed to send to StateService");
    state_service_tx
        .send(services::state_service::Message::CreateInstance(1, 1))
        .expect("failed to send to StateService");

    info!("Server initialized");

    state_service.join().unwrap();
    level_service.join().unwrap();
    sender_service.join().unwrap();
    receiver_service.join().unwrap();
}
