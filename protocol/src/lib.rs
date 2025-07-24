use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong { ping_time_elapsed: Duration },
    LoadScene {
        scene_name: String
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping { time_elapsed: Duration },
}
