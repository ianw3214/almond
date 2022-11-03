use specs::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};

use crate::{components::*, DeltaTime, camera};

const TILES_ACROSS: i32 = 4;

struct RenderTarget {
    y: i32,
    src: Option<Rect>,
    dst: Rect,
    texture_index: usize
}

pub fn render(canvas: &mut WindowCanvas, textures: &Vec<Texture>, world: &World) {
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
    let mut animatables = world.write_storage::<Animatable>();
    let camera_info = world.read_resource::<camera::Camera>();
    let delta = world.read_resource::<DeltaTime>();

    // TODO: Priority queue/more efficient way to render
    let mut render_targets : Vec<RenderTarget> = Vec::new();

    for (pos, render, animatable) in (&positions, &renderables, (&mut animatables).maybe()).join() {
        let mut src_rect: Option<Rect> = None;
        if let Some(animatable) = animatable {
            let x = animatable.frame % TILES_ACROSS;
            let y = animatable.frame / TILES_ACROSS;
            let w = animatable.width;
            let h = animatable.height;
            src_rect = Some(Rect::new(x * w, y * h, w as u32, h as u32));
            animatable.timer = animatable.timer + delta.0;
            // Update animation frame based on fps
            if animatable.timer > 1.0 / 12.0 {
                animatable.frame = (animatable.frame + 1) % 4;
                animatable.timer = 0.0;
            }
        }
        let screen_x = camera_info.world_to_screen_x(pos.x);
        let screen_y = camera_info.world_to_screen_y(pos.y);
        let screen_rect = Rect::new(screen_x, screen_y, 64, 64);
        render_targets.push(RenderTarget{ y : screen_rect.y, src : src_rect, dst : screen_rect, texture_index : render.i });
    }

    render_targets.sort_by(|a, b| a.y.cmp(&b.y));
    for target in render_targets {
        canvas.copy(&textures[target.texture_index], target.src, target.dst).expect("render copy failed...");
    }

}