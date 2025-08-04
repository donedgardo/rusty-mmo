use bevy::app::{App, Startup, Update};
use bevy::log::LogPlugin;
use bevy::prelude::{AssetPlugin, default, ImagePlugin, ResMut, TransformPlugin};
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use std::net::Ipv6Addr;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::MinimalPlugins;
use bevy::pbr::PbrPlugin;
use bevy::render::RenderPlugin;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::window::WindowPlugin;
use world::WorldPlugin;
use protocol::{ClientMessage, ServerMessage};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            TransformPlugin::default(),
            WindowPlugin::default(),
            RenderPlugin{
                synchronous_pipeline_compilation: true,
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: None,
                    ..default()
                }),
                debug_flags: Default::default(),
            },
            ImagePlugin::default(),
            CorePipelinePlugin::default(),
            PbrPlugin::default(),
            LogPlugin::default(),
            QuinnetServerPlugin::default(),
            WorldPlugin
        ))
        .add_systems(Startup, start_listening)
        .add_systems(Update, handle_client_messages)
        .run();
}

fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_ip(Ipv6Addr::UNSPECIFIED, 6000),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: Ipv6Addr::LOCALHOST.to_string(),
            },
            ChannelsConfiguration::default(),
        )
        .unwrap();
}

fn handle_client_messages(mut server: ResMut<QuinnetServer>) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some((_channel_id, message)) =
            endpoint.try_receive_message_from::<ClientMessage>(client_id)
        {
            match message {
                ClientMessage::Ping {
                    time_elapsed: time_delta,
                } => {
                    endpoint
                        .send_message(
                            client_id,
                            ServerMessage::Pong {
                                ping_time_elapsed: time_delta,
                            },
                        )
                        .expect("Error ponging.");
                }
            }
        }
    }
}
