use std::{env, str::FromStr};

pub const DEFAULT_PORT: u16 = 1337;
pub const DEFAULT_SYNC_INTERVAL_MS: u64 = 40;

// Temporary directory to load level data from
pub const DEFAULT_LEVEL_DATA_DIR: &str = "./level-data";

#[inline(always)]
pub fn load_env_or<T: FromStr>(var: &str, default: T) -> T {
    env::var(var)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}
