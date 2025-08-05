use bevy::prelude::Event;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong { ping_time_elapsed: Duration },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping { time_elapsed: Duration },
}

#[derive(Event, Debug, Clone)]
pub struct ClientMessageReceived {
    pub client_id: u64,
    pub message: ClientMessage,
}

#[derive(Event, Debug, Clone)]
pub struct ServerMessageReceived {
    pub message: ServerMessage,
}
