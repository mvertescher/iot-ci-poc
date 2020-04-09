//! Client/server websocket protocol

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Log
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
    data: String,
}


