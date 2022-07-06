use specs::prelude::*;

use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Clickable>,
    ReadStorage<'a, WorldPosition>
);

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;

    for (clickable, pos) in (&data.0, &data.1).join() {
        // Render the selected sprite over selected entity
        if clickable.selected {
            let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, 40, 40);
            canvas.copy(&textures[0], None, screen_rect)?;
        }
    }

    Ok(())
}