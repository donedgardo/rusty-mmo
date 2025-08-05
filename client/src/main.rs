use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ClientEndpointConfiguration;
use bevy_quinnet::client::{QuinnetClient, QuinnetClientPlugin};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use ping::ClientPingPlugin;
use std::net::Ipv6Addr;
use protocol::{ServerMessageReceived};
use world::WorldPlugin;

fn main() {
    App::new()
      .add_event::<ServerMessageReceived>()
      .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../assets".to_string(),
                ..default()
            }),
            QuinnetClientPlugin::default(),
            ClientPingPlugin,
            WorldPlugin,
        ))
        .add_systems(Startup, (start_connection, setup_camera_and_global_lights))
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

fn setup_camera_and_global_lights(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 16.0, 40.0).looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(5.0, 5.0, 5.0)));
    commands.spawn((
        DirectionalLight {
            illuminance: 4000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::default().looking_at(-Vec3::Y, Vec3::Z),
    ));
}
