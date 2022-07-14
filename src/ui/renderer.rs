use specs::prelude::*;

use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;
use crate::util::*;
use crate::{ScreenInfo, CameraInfo, GridSize, MouseInfo};

pub type SystemData<'a> = (
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, CameraInfo>,
    ReadExpect<'a, MouseInfo>,
    ReadExpect<'a, GridSize>,
    ReadStorage<'a, Selectable>,
    ReadStorage<'a, WorldPosition>,
    ReadStorage<'a, Sprite>
);

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    let screen_info = &*data.0;
    let camera = &*data.1;
    for (selectable, pos, sprite) in (&data.4, &data.5, &data.6).join() {
        // Render the selected sprite over selected entity
        if selectable.selected {
            let screen_position = world_to_screen_pos(screen_info, camera, pos.point);
            let screen_rect = Rect::new(
                screen_position.x + (sprite.x_offset * camera.scale as i32), 
                screen_position.y + (sprite.y_offset * camera.scale as i32), 
                (40.0 * camera.scale) as u32, 
                (40.0 * camera.scale) as u32
            );
            canvas.copy(&textures[0], None, screen_rect)?;
        }
    }

    // Draw the hovered grid
    let mouse_info = &*data.2;
    let grid_size = &*data.3;
    let grid_pos = screen_to_grid_pos(&*data.0, camera, grid_size, Point::new(mouse_info.x, mouse_info.y));
    let final_point = Point::new(
        grid_pos.x * grid_size.width,
        grid_pos.y * grid_size.height
    );
    let screen_position = world_to_screen_pos(screen_info, camera, final_point);
    let screen_rect = Rect::new(
        screen_position.x,
        screen_position.y,
        (40.0 * camera.scale) as u32,
        (40.0 * camera.scale) as u32
    );
    canvas.copy(&textures[1], None, screen_rect)?;

    Ok(())
}