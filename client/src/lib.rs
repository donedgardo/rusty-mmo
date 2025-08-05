use bevy::prelude::{EventWriter, ResMut};
use bevy_quinnet::client::QuinnetClient;
use protocol::{ServerMessage, ServerMessageReceived};

pub fn handle_server_messages(
    mut client: ResMut<QuinnetClient>,
    mut event_writer: EventWriter<ServerMessageReceived>,
) {
    while let Ok(Some((_channel_id, message))) =
        client.connection_mut().receive_message::<ServerMessage>()
    {
        event_writer.write(ServerMessageReceived { message });
    }
}
