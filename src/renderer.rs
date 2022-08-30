use specs::prelude::*;

use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

pub fn render(canvas: &mut WindowCanvas, textures: &[Texture], world: &World) {
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();

    for (pos, render) in (&positions, &renderables).join() {
        let screen_rect = Rect::from_center(Point::new(pos.x, pos.y), 64, 64);
        canvas.copy(&textures[render.i], None, screen_rect).expect("render copy failed...");
    }
}