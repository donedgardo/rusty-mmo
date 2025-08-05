use bevy::log::info;
use bevy::prelude::{Res, ResMut, Time};
use bevy_quinnet::client::QuinnetClient;
use protocol::ServerMessage;

pub fn handle_server_messages(mut client: ResMut<QuinnetClient>, time: Res<Time>) {
    while let Ok(Some((_channel_id, message))) =
        client.connection_mut().receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::Pong {
                ping_time_elapsed: ping_time_delta,
            } => {
                info!("Ping: {:?}", time.elapsed() - ping_time_delta);
            }
        }
    }
}
