use bevy::{math, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct PhysicMeshBundle {
    rigid_body: RigidBody,
    force: ExternalForce,
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    restitution: Restitution,
}

impl PhysicMeshBundle {
    pub fn dynamic_circle(
        radius: f32,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let material_mesh = MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(radius)).into(),
            material: materials.add(Color::srgb(15.25, 9.4, 9.1)), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform {
                translation: math::vec3(0., 0., 2.),
                ..default()
            },
            ..default()
        };
        let collider = Collider::ball(radius + 50.);
        let restitution = Restitution::coefficient(0.7);

        PhysicMeshBundle {
            rigid_body: RigidBody::Dynamic,
            force: ExternalForce{ force: Vec2::new(0.0, -200.0), torque: 0. },
            material_mesh,
            collider,
            restitution,
        }
    }
}
