use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::{
    AdditionalMassProperties, ColliderMassProperties, ExternalForce, Velocity,
};

use crate::physic_mesh_bundle::PhysicMeshBundle;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    physic: PhysicMeshBundle,
}

/// Player movement speed factor.
const PLAYER_ACCELERATION: f32 = 10.;
const PLAYER_MAX_SPEED: f32 = 300.;

#[derive(Component, Clone, Copy)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_bundle = PlayerBundle {
        player: Player,
        physic: PhysicMeshBundle::dynamic_circle(25., meshes, materials),
    };
    commands
        .spawn(player_bundle)
        //.insert(ColliderMassProperties::Density(2.0))
        .insert(AdditionalMassProperties::Mass(2.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        });
}

/// Update the player position with keyboard inputs.
pub fn move_player(
    //mut player: Query<&mut Transform, With<Player>>,
    mut velocity: Query<&mut Velocity, With<Player>>,
    //mut grav_scale: Query<&mut GravityScale>,
    //time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    //let mut direction = Vec2::ZERO;
    let mut velocity = velocity.single_mut();

    if kb_input.pressed(KeyCode::Space) {
        //direction.y += 1.;
        velocity.linvel += Vec2::new(0.0, PLAYER_ACCELERATION * 2.);
        //dbg!(velocity.linvel);
    }
    if kb_input.pressed(KeyCode::KeyS) {
        velocity.linvel += Vec2::new(0.0, -PLAYER_ACCELERATION);
    }

    if kb_input.pressed(KeyCode::KeyA) {
        velocity.linvel += Vec2::new(-PLAYER_ACCELERATION, 0.0);
    }

    if kb_input.pressed(KeyCode::KeyD) {
        velocity.linvel += Vec2::new(PLAYER_ACCELERATION, 0.0);
    }

    let speed_mod = velocity.linvel.distance(Vec2::ZERO);
    dbg!(speed_mod);
    if let (true, Some(vel)) = (
        speed_mod > PLAYER_MAX_SPEED,
        velocity.linvel.try_normalize(),
    ) {
        velocity.linvel = vel * PLAYER_MAX_SPEED;
    }
}
