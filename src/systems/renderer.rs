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

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    for (pos, sprite) in (&data.2, &data.3).join() {
        let current_frame = sprite.region;
        let camera = &*data.1;
        
        let screen_position = world_to_screen_pos(&*data.0, camera, pos.0);
        let screen_rect = Rect::new(
            screen_position.x, 
            screen_position.y, 
            ((current_frame.width() as f32) * camera.scale) as u32,
            ((current_frame.height() as f32) * camera.scale) as u32
        );    
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    Ok(())
}