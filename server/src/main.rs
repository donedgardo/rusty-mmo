use bevy::app::{App, ScheduleRunnerPlugin, Startup, Update};
use bevy::log::LogPlugin;
use bevy::prelude::ResMut;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use protocol::{ClientMessage, ServerMessage};
use std::net::Ipv6Addr;

fn main() {
    App::new()
        .add_plugins((
            ScheduleRunnerPlugin::default(),
            LogPlugin::default(),
            QuinnetServerPlugin::default(),
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
