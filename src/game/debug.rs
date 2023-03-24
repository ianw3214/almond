use bevy::prelude::{Transform, Query, Component, With};

#[derive(Component)]
pub struct DebugCollision;

pub fn draw_collisions(
    mut collisions : Query<&mut Transform, With<DebugCollision>>
) {
    for mut collision in collisions.iter_mut() {
        // TODO: Set this size based on a collision box
        collision.translation.x = 0.0;
        collision.translation.y = 0.0;
        collision.translation.z = 1.0;
    }
}