//! Client/server websocket protocol

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Log(Log)
}

impl Message {
    pub fn log(data: String) -> Message {
        Message::Log(Log { data })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    data: String,
}


