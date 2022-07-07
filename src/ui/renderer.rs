use specs::prelude::*;

use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;
use crate::util::*;
use crate::ScreenInfo;

pub type SystemData<'a> = (
    ReadExpect<'a, ScreenInfo>,
    ReadStorage<'a, Clickable>,
    ReadStorage<'a, WorldPosition>
);

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    for (clickable, pos) in (&data.1, &data.2).join() {
        // Render the selected sprite over selected entity
        if clickable.selected {
            let screen_position = window_to_screen_pos(&*data.0, pos.0);
            let screen_rect = Rect::new(screen_position.x, screen_position.y, 40, 40);
            canvas.copy(&textures[0], None, screen_rect)?;
        }
    }

    Ok(())
}