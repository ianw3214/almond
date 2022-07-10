use specs::prelude::*;

use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;
use crate::util::*;
use crate::{ScreenInfo, CameraInfo};

pub type SystemData<'a> = (
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, CameraInfo>,
    ReadStorage<'a, Selectable>,
    ReadStorage<'a, WorldPosition>
);

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    let camera = &*data.1;
    for (clickable, pos) in (&data.2, &data.3).join() {
        // Render the selected sprite over selected entity
        if clickable.selected {
            let screen_position = world_to_screen_pos(&*data.0, camera, pos.0);
            let screen_rect = Rect::new(
                screen_position.x, 
                screen_position.y, 
                (40.0 * camera.scale) as u32, 
                (40.0 * camera.scale) as u32
            );
            canvas.copy(&textures[0], None, screen_rect)?;
        }
    }

    Ok(())
}