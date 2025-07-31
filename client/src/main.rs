use bevy::prelude::{App, AssetPlugin, Commands, default, DefaultPlugins, PluginGroup, Res, ResMut, SceneRoot, Startup, Update};
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ClientEndpointConfiguration;
use bevy_quinnet::client::{QuinnetClient, QuinnetClientPlugin};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use std::net::Ipv6Addr;
use bevy::asset::AssetServer;
use bevy::gltf::GltfAssetLabel;
use ping::PingPlugin;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../assets".to_string(),
                ..default()
            }),
            QuinnetClientPlugin::default(),
            PingPlugin,
            bevy_skein::SkeinPlugin::default(),
        )).add_systems(Startup, (start_connection, load_word))
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

fn load_word(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("00.gltf"),
    )));
}


