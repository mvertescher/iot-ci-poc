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

    pub fn encode(self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }

    pub fn decode(data: &[u8]) -> Result<Self, ()> {
        let msg: Message = serde_cbor::from_slice(data).unwrap();
        Ok(msg)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    data: String,
}


