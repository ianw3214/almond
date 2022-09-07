use specs::prelude::*;

use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

const TILES_ACROSS: i32 = 4;

pub fn render(canvas: &mut WindowCanvas, textures: &[Texture], world: &World) {
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
    let mut animatables = world.write_storage::<Animatable>();

    for (pos, render, animatable) in (&positions, &renderables, (&mut animatables).maybe()).join() {
        let mut src_rect: Option<Rect> = None;
        if let Some(animatable) = animatable {
            let x = animatable.frame % TILES_ACROSS;
            let y = animatable.frame / TILES_ACROSS;
            let w = animatable.width;
            let h = animatable.height;
            src_rect = Some(Rect::new(x * w, y * h, w as u32, h as u32));
            animatable.frame = (animatable.frame + 1) % 4;
        }
        let screen_rect = Rect::from_center(Point::new(pos.x, pos.y), 64, 64);
        canvas.copy(&textures[render.i], src_rect, screen_rect).expect("render copy failed...");
    }
}