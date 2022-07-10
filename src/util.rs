use sdl2::rect::{Point /*, Rect  */};

use crate::{ScreenInfo, CameraInfo};

pub fn world_to_screen_pos(screen: &ScreenInfo, camera: &CameraInfo, point: Point) -> Point {
    Point::new(
        (point.x as f32 * camera.scale) as i32 + screen.width / 2,
        (point.y as f32 * camera.scale) as i32  + screen.height / 2
    )
}

/*
pub fn windowToScreenRect(canvas: &mut WindowCanvas, region: Rect) -> Rect{
    let (width, height) = canvas.output_size().expect("thing");

    Rect::new(
        region.x + width as i32 / 2,
        region.y + height as i32 / 2,
        region.w as u32,
        region.h as u32
    )
}
*/

pub fn screen_to_world_pos(screen: &ScreenInfo, camera: &CameraInfo, point: Point) -> Point {
    Point::new(
        ((point.x as i32 - screen.width / 2) as f32 / camera.scale) as i32,
        ((point.y as i32 - screen.height / 2) as f32 / camera.scale) as i32
    )
}