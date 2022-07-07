use specs::prelude::*;

use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;
use crate::util::*;
use crate::ScreenInfo;

pub type SystemData<'a> = (
    ReadExpect<'a, ScreenInfo>,
    ReadStorage<'a, WorldPosition>,
    ReadStorage<'a, Sprite>
);

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    for (pos, sprite) in (&data.1, &data.2).join() {
        let current_frame = sprite.region;
        
        let screen_position = window_to_screen_pos(&*data.0, pos.0);
        let screen_rect = Rect::new(screen_position.x, screen_position.y, current_frame.width(), current_frame.height());    
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    Ok(())
}