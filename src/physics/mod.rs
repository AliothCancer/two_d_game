
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Bundle)]
struct PhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    transform_bundle: TransformBundle,
    mesh: Handle<Mesh>
}




