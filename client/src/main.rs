use bevy::prelude::{App, DefaultPlugins, ResMut, Startup, Update};
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ClientEndpointConfiguration;
use bevy_quinnet::client::{QuinnetClient, QuinnetClientPlugin};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use std::net::Ipv6Addr;
use ping::PingPlugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, QuinnetClientPlugin::default(), PingPlugin))
        .add_systems(Startup, start_connection)
        .add_systems(Update, client::handle_server_messages)
        .run();
}

fn start_connection(mut client: ResMut<QuinnetClient>) {
    client
        .open_connection(
            ClientEndpointConfiguration::from_ips(
                Ipv6Addr::LOCALHOST,
                6000,
                Ipv6Addr::UNSPECIFIED,
                0,
            ),
            CertificateVerificationMode::SkipVerification,
            ChannelsConfiguration::default(),
        )
        .expect("Error connecting to server.");
}

