use bevy::MinimalPlugins;
use bevy::app::{App, Startup, Update};
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::{AssetPlugin, EventWriter, ImagePlugin, ResMut, TransformPlugin, default};
use bevy::render::RenderPlugin;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::window::WindowPlugin;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use ping::ServerPingPlugin;
use protocol::{ClientMessage, ClientMessageReceived};
use std::net::Ipv6Addr;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_event::<ClientMessageReceived>()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            TransformPlugin::default(),
            WindowPlugin::default(),
            RenderPlugin {
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
            WorldPlugin,
            ServerPingPlugin,
        ))
        .add_systems(Startup, start_listening)
        .add_systems(Update, receive_client_messages)
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

fn receive_client_messages(
    mut server: ResMut<QuinnetServer>,
    mut event_writer: EventWriter<ClientMessageReceived>,
) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some((_channel_id, message)) =
            endpoint.try_receive_message_from::<ClientMessage>(client_id)
        {
            event_writer.write(ClientMessageReceived { client_id, message });
        }
    }
}
