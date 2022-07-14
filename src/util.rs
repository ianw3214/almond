use sdl2::rect::{Point /*, Rect  */};

use crate::{ScreenInfo, CameraInfo, GridSize};

pub fn world_to_screen_pos(screen: &ScreenInfo, camera: &CameraInfo, point: Point) -> Point {
    Point::new(
        (point.x as f32 * camera.scale) as i32 + screen.width / 2,
        (point.y as f32 * camera.scale) as i32  + screen.height / 2
    )
}

pub fn screen_to_world_pos(screen: &ScreenInfo, camera: &CameraInfo, point: Point) -> Point {
    Point::new(
        ((point.x as i32 - screen.width / 2) as f32 / camera.scale) as i32,
        ((point.y as i32 - screen.height / 2) as f32 / camera.scale) as i32
    )
}

pub fn screen_to_grid_pos(screen: &ScreenInfo, camera: &CameraInfo, grid: &GridSize, point: Point) -> Point {
    let world_x = ((point.x as i32 - screen.width / 2) as f32 / camera.scale) as i32;
    let world_y = ((point.y as i32 - screen.height / 2) as f32 / camera.scale) as i32;
    Point::new(
        world_x / grid.width - if world_x < 0 { 1 } else { 0 },
        world_y / grid.height - if world_y < 0 { 1 } else { 0 }
    )
}