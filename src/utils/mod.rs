use crate::constants::config::CONFIG_JSON_PATH;

use self::json::{read_json, write_json};

use rand::{rngs::ThreadRng, thread_rng};
use serde_json::json;

pub mod battle_data;
pub mod battle_replay;
pub mod comp;
pub mod crypto;
pub mod fmt;
pub mod game;
pub mod json;
pub mod mail;
pub mod random;
pub mod rlv2;
pub mod server;
pub mod update;

pub fn zip<T: IntoIterator, U: IntoIterator>(a: T, b: U) -> Vec<(T::Item, U::Item)> {
    a.into_iter().zip(b).collect()
}

pub fn enumerate<T: IntoIterator>(a: T) -> Vec<(usize, T::Item)> {
    a.into_iter().enumerate().collect()
}

pub fn get_nickname_config() -> (String, String) {
    let config = read_json(CONFIG_JSON_PATH);
    let mut mut_conf = config.clone();
    let nick_name = match config["userConfig"]["nickName"].as_str() {
        Some(nick_name) => nick_name,
        None => {
            mut_conf["userConfig"]["nickName"] = json!("Terra");
            write_json(CONFIG_JSON_PATH, &mut_conf);
            "Terra"
        }
    };
    let nick_id = match config["userConfig"]["nickNumber"].as_str() {
        Some(nick_name) => nick_name,
        None => {
            mut_conf["userConfig"]["nickNumber"] = json!("1111");
            write_json(CONFIG_JSON_PATH, &mut_conf);
            "1111"
        }
    };
    (nick_name.to_string(), nick_id.to_string())
}

pub struct TRng(pub ThreadRng);
impl TRng {
    pub fn new() -> Self {
        Self(thread_rng())
    }
}

unsafe impl Send for TRng {}
