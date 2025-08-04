use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level);
    }
}

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the ground.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(128.0, 128.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));
    // Spawn a little platform for the player to jump on.
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 1.0, 4.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(-6.0, 2.0, 0.0),
    ));
}