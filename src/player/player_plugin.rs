use bevy::prelude::*;
use bevy_rapier2d::prelude::{AdditionalMassProperties, ColliderMassProperties, ExternalForce, Velocity};

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
const PLAYER_SPEED: f32 = 200.;

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
    commands.spawn(player_bundle)
    //.insert(ColliderMassProperties::Density(2.0))
    .insert(AdditionalMassProperties::Mass(2.0))
    .insert(Velocity {
        linvel: Vec2::new(0.0, 0.0),
        angvel: 0.0,
    });

}

/// Update the player position with keyboard inputs.
pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    mut velocity: Query<&mut Velocity, With<Player>>,
    //mut grav_scale: Query<&mut GravityScale>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::Space) {
        //direction.y += 1.;
        let mut velocity = velocity.single_mut();
        velocity.linvel += Vec2::new(0.0, 20.0);
        dbg!(velocity.linvel);
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player.translation += move_delta.extend(0.);
}
