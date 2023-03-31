use bevy::prelude::{Query, ResMut, Vec3};
use bevy_prototype_debug_lines::DebugLines;

use super::components::{WorldPosition, Collision};

pub fn draw_collisions(
    collisions : Query<(&WorldPosition, &Collision)>,
    // windows : Res<Windows>,
    mut lines : ResMut<DebugLines>
) {
    // TODO: Should this be stored and cached at the beginning of a tick?
    // let window_width = windows.get_primary().unwrap().width();
    // let window_height = windows.get_primary().unwrap().height();
    for (pos, collision) in collisions.iter() {
        // let screen_x = pos.x / window_width;
        // let screen_y = pos.y / window_height;
        // let screen_x2 = (pos.x + collision.width) / window_width;
        // let screen_y2 = (pos.y + collision.height) / window_height;
        let screen_x = pos.x;
        let screen_y = pos.y;
        let screen_x2 = pos.x + collision.width;
        let screen_y2 = pos.y + collision.height;
        let top_left = Vec3::new(screen_x, screen_y, 1.0);
        let top_right = Vec3::new(screen_x2, screen_y, 1.0);
        let bottom_left = Vec3::new(screen_x, screen_y2, 1.0);
        let bottom_right = Vec3::new(screen_x2, screen_y2, 1.0);
        lines.line(top_left, top_right, 0.0);
        lines.line(top_left, bottom_left, 0.0);
        lines.line(top_right, bottom_right, 0.0);
        lines.line(bottom_left, bottom_right, 0.0);
    }
}