use specs::prelude::*;

use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::components::*;

pub fn render(canvas: &mut WindowCanvas, world: &World) {
    let positions = world.read_storage::<Position>();
    let aabbs = world.write_storage::<BoundingBox>();

    for (pos, aabb) in (&positions, &aabbs).join() {
        let x = pos.x + aabb.x_offset;
        let y = pos.y + aabb.y_offset;
        let w = aabb.width;
        let h = aabb.height;
        let top_left = Point::new(x, y);
        let top_right = Point::new(x + w as i32, y);
        let bot_left = Point::new(x, y + h as i32);
        let bot_right = Point::new(x + w as i32, y + h as i32);
        canvas.draw_line(top_left, top_right).expect("");
        canvas.draw_line(top_left, bot_left).expect("");
        canvas.draw_line(bot_right, top_right).expect("");
        canvas.draw_line(bot_right, bot_left).expect("");
    }
}