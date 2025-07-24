use bevy::log::info;
use bevy::prelude::{Res, ResMut, Resource, Time};
use bevy_quinnet::client::QuinnetClient;
use protocol::{ServerMessage};

#[derive(Resource, Default)]
pub struct SceneLoader {
    pub scenes_loaded: Vec<String>,
}

impl SceneLoader {
    pub fn load_scene(&mut self, scene_name: String) {
        info!("Loading scene: {}", scene_name);
        self.scenes_loaded.push(scene_name);
    }
}

pub fn handle_server_messages(mut client: ResMut<QuinnetClient>, time: Res<Time>) {
    while let Ok(Some((_channel_id, message))) =
        client.connection_mut().receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::Pong { ping_time_elapsed: ping_time_delta } => {
                info!("Ping: {:?}", time.elapsed() - ping_time_delta);
            }
            ServerMessage::LoadScene { .. } => {}
        }
    }
}

