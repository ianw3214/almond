use specs::prelude::*;

use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;
use crate::util::*;
use crate::{ScreenInfo, CameraInfo};

pub type SystemData<'a> = (
    // Global resources
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, CameraInfo>,
    // Components
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
    // Global resources
    let screen_info = &*data.0;
    let camera_info = &*data.1;
    // Components
    let world_positions = &data.2;
    let sprites = &data.3;

    // TODO: Priority queue/more efficient way to render
    let mut render_targets : Vec<RenderTarget> = Vec::new();

    for (pos, sprite) in (world_positions, sprites).join() {
        let current_frame = sprite.region;
        
        // TODO: These calculations can be refactored
        // - currently shared with ui renderer for selected sprite
        let screen_position = world_to_screen_pos(screen_info, camera_info, pos.point);
        let screen_rect = Rect::new(
            screen_position.x + (sprite.x_offset * camera_info.scale as i32), 
            screen_position.y + (sprite.y_offset * camera_info.scale as i32), 
            ((current_frame.width() as f32) * camera_info.scale) as u32,
            ((current_frame.height() as f32) * camera_info.scale) as u32
        );
        render_targets.push(RenderTarget{ y: screen_rect.y, rect: screen_rect, sprite });
    }

    render_targets.sort_by(|a, b| a.y.cmp(&b.y));
    for target in render_targets {
        canvas.copy(&textures[target.sprite.spritesheet], target.sprite.region, target.rect)?;
    }

    Ok(())
}