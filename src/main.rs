mod game_abstractions;
mod physic_mesh_bundle;
mod player;
mod world;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::camera_plugin::CameraPlugin;
use player::player_plugin::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup_instructions)
        .run();
}


fn setup_instructions(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Move the light with ZQSD or WASD.\nThe camera will smoothly track the light.",
            TextStyle::default(),
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}
