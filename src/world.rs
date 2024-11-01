use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::prelude::Collider;


pub struct WorldPlugin;
impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_world);
    }
}



fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // World where we move the player
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(1000., 700.))),
        material: materials.add(Color::srgb(0.2, 0.2, 0.6)),
        ..default()
    });

    // terrain collision
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}
