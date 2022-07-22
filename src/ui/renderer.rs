use specs::prelude::*;

use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;
use crate::util::*;
use crate::{ScreenInfo, CameraInfo, GridSize, MouseInfo};

pub type SystemData<'a> = (
    // Global resources
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, CameraInfo>,
    ReadExpect<'a, MouseInfo>,
    ReadExpect<'a, GridSize>,
    // Components
    ReadStorage<'a, Selectable>,
    ReadStorage<'a, WorldPosition>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, Turn>
);

pub fn render(
    canvas: &mut WindowCanvas, 
    textures: &[Texture],
    data: SystemData
) -> Result<(), String> {
    // Global resources
    let screen_info = &*data.0;
    let camera_info = &*data.1;
    let mouse_info = &*data.2;
    let grid_size = &*data.3;
    // Components
    let selectables = &data.4;
    let world_positions = &data.5;
    let sprites = &data.6;
    let turns = &data.7;

    for (selectable, pos, sprite, turn) in (selectables, world_positions, sprites, turns.maybe()).join() {
        // Render the selected sprite over selected entity
        if selectable.selected {
            let screen_position = world_to_screen_pos(screen_info, camera_info, pos.point);
            let screen_rect = Rect::new(
                screen_position.x + (sprite.x_offset * camera_info.scale as i32), 
                screen_position.y + (sprite.y_offset * camera_info.scale as i32), 
                (40.0 * camera_info.scale) as u32, 
                (40.0 * camera_info.scale) as u32
            );
            canvas.copy(&textures[0], None, screen_rect)?;
        }
        // Render the turn sprite over current turn entity
        if let Some(turn) = turn {
            if turn.current {
                let screen_position = world_to_screen_pos(screen_info, camera_info, pos.point);
                let screen_rect = Rect::new(
                    screen_position.x + (sprite.x_offset * camera_info.scale as i32), 
                    screen_position.y - (20 * camera_info.scale as i32), 
                    (40.0 * camera_info.scale) as u32, 
                    (40.0 * camera_info.scale) as u32
                );
                canvas.copy(&textures[2], None, screen_rect)?;
            }
        }
    }

    // Draw the hovered grid
    let grid_pos = screen_to_grid_pos(screen_info, camera_info, grid_size, Point::new(mouse_info.x, mouse_info.y));
    let final_point = Point::new(
        grid_pos.x * grid_size.width,
        grid_pos.y * grid_size.height
    );
    let screen_position = world_to_screen_pos(screen_info, camera_info, final_point);
    let screen_rect = Rect::new(
        screen_position.x,
        screen_position.y,
        (40.0 * camera_info.scale) as u32,
        (40.0 * camera_info.scale) as u32
    );
    canvas.copy(&textures[1], None, screen_rect)?;

    Ok(())
}