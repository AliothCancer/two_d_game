use bevy::{math, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 200.;

#[derive(Component, Clone, Copy)]
pub struct Player {
    radius: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = Player { radius: 25. };
    commands
        .spawn((
            player,
            RigidBody::Dynamic,
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(player.radius)).into(),
                material: materials.add(Color::srgb(6.25, 9.4, 9.1)), // RGB values exceed 1 to achieve a bright color for the bloom effect
                transform: Transform {
                    translation: math::vec3(0., 0., 2.),
                    ..default()
                },
                ..default()
            },
        ))
        .insert(Restitution::coefficient(0.7))
        .insert(Collider::ball(player.radius));
}

/// Update the player position with keyboard inputs.
pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    //mut grav_scale: Query<&mut GravityScale>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::Space) {
        direction.y += 1.;
        //grav_scale.single_mut().0 = 0.1;
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
