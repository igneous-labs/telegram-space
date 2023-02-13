use std::time::Duration;

pub const PORT: u16 = 1337;
pub const WORLD_STATE_SYNC_INTERVAL: Duration = Duration::from_millis(40);

// Temporary directory to load level data from
pub const LEVEL_DATA_DIR: &str = "./level-data";
