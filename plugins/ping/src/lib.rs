use bevy::app::{App, Plugin, Update};
use bevy::prelude::{EventReader, Res, ResMut, Resource, Time, Timer, TimerMode};
use bevy_quinnet::client::QuinnetClient;
use bevy_quinnet::server::QuinnetServer;
use protocol::{ClientMessage, ClientMessageReceived, ServerMessage};

pub struct ClientPingPlugin;

impl Plugin for ClientPingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PingTimer>()
            .add_systems(Update, ping_system);
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
        .send_message(ClientMessage::Ping {
            time_elapsed: time_delta,
        })
        .expect("Error sending Ping");
}
pub struct ServerPingPlugin;

impl Plugin for ServerPingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_ping_messages);
    }
}

fn handle_ping_messages(
    mut server: ResMut<QuinnetServer>,
    mut events: EventReader<ClientMessageReceived>,
) {
    for event in events.read() {
        match event.message {
            ClientMessage::Ping {
                time_elapsed: time_delta,
            } => {
                let endpoint = server.endpoint_mut();
                endpoint
                  .send_message(
                      event.client_id,
                      ServerMessage::Pong {
                          ping_time_elapsed: time_delta,
                      },
                  )
                  .expect("Error ponging.");
            }
        }
    }
}
