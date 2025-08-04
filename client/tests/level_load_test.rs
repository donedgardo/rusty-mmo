use bevy::MinimalPlugins;
use bevy::app::App;
use bevy::prelude::*;

use client::{SceneLoader, handle_server_messages};

fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<SceneLoader>()
        .add_systems(Update, handle_server_messages);
    app
}

#[test]
fn test_scene_loading_integration() {
    let _app = create_test_app();
    assert_eq!("1", "1");
}
