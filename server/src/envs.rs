use std::{env, str::FromStr};

pub const DEFAULT_PORT: u16 = 1337;
pub const DEFAULT_SYNC_INTERVAL_MS: u64 = 40;

// Temporary directory to load level data from
pub const DEFAULT_LEVEL_DATA_DIR: &str = "./level-data";
pub const DEFAULT_RUST_LOG: &str = "telegram_space_server=debug";
pub const DEFAULT_LOG_DIR: &str = "./logs";

#[inline(always)]
pub fn parse_env_or<T: FromStr>(var: &str, default: T) -> T {
    env::var(var)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}
