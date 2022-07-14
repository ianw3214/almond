use specs::prelude::*;

use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;
use crate::util::*;
use crate::{ScreenInfo, CameraInfo};

pub type SystemData<'a> = (
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, CameraInfo>,
    ReadStorage<'a, WorldPosition>,
    ReadStorage<'a, Sprite>
);

struct RenderTarget<'a> {
    y: i32,
    rect: Rect,
    sprite: &'a Sprite
}

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    // TODO: Priority queue/more efficient way to render
    let mut render_targets : Vec<RenderTarget> = Vec::new();

    for (pos, sprite) in (&data.2, &data.3).join() {
        let screen_info = &*data.0;
        let camera = &*data.1;
        let current_frame = sprite.region;
        
        // TODO: These calculations can be refactored
        // - currently shared with ui renderer for selected sprite
        let screen_position = world_to_screen_pos(screen_info, camera, pos.point);
        let screen_rect = Rect::new(
            screen_position.x + (sprite.x_offset * camera.scale as i32), 
            screen_position.y + (sprite.y_offset * camera.scale as i32), 
            ((current_frame.width() as f32) * camera.scale) as u32,
            ((current_frame.height() as f32) * camera.scale) as u32
        );
        render_targets.push(RenderTarget{ y: screen_rect.y, rect: screen_rect, sprite });
    }

    render_targets.sort_by(|a, b| a.y.cmp(&b.y));
    for target in render_targets {
        canvas.copy(&textures[target.sprite.spritesheet], target.sprite.region, target.rect)?;
    }

    Ok(())
}