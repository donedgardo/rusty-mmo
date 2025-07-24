use bevy::prelude::{Res, ResMut, Resource, Time, Timer, TimerMode};
use bevy_quinnet::client::QuinnetClient;
use bevy::app::{App, Plugin, Update};
use protocol::ClientMessage;

pub struct PingPlugin;

impl Plugin for PingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PingTimer>()
            .add_systems(Update,  ping_system);
    }
}

#[derive(Resource)]
struct PingTimer(Timer);

impl Default for PingTimer {
    fn default() -> PingTimer {
        PingTimer(Timer::from_seconds(1., TimerMode::Repeating))
    }
}

fn ping_system(time: Res<Time>, mut client: ResMut<QuinnetClient>, mut timer: ResMut<PingTimer>) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let time_delta = time.elapsed();
    let connection = client.connection_mut();
    connection
        .send_message(ClientMessage::Ping { time_elapsed: time_delta })
        .expect("Error sending Ping");
}
