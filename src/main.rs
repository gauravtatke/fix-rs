#![allow(dead_code, unused_variables)]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

mod application;
mod data_dictionary;
mod io;
mod message;
mod network;
mod quickfix_errors;
mod session;

use crate::session::SessionProperties;

pub(crate) const FILE_PATH: &str = "resources/FIX43.xml";
pub(crate) const FIX_CONFIG_PATH: &str = "src/FixCfg.toml";

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let settings_str = std::fs::read_to_string(FIX_CONFIG_PATH).unwrap();
    let _ = SessionProperties::from_str(&settings_str);
}
