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
    let scaled_world_x = point.x as i32 - screen.width / 2;
    let scaled_world_y = point.y as i32 - screen.height / 2;
    let world_x = (scaled_world_x as f32 / camera.scale) as i32;
    let world_y = (scaled_world_y as f32/ camera.scale) as i32;
    Point::new(
        world_x / grid.width - if scaled_world_x < 0 { 1 } else { 0 },
        world_y / grid.height - if scaled_world_y < 0 { 1 } else { 0 }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const SCREEN_WIDTH : i32 = 500;
    const SCREEN_HEIGHT : i32 = 500;
    const CAMERA_SCALE : f32 = 2.0;
    const GRID_WIDTH : i32 = 10;
    const GRID_HEIGHT : i32 = 10;

    fn setup_info() -> (ScreenInfo, CameraInfo, GridSize) {
        (
            ScreenInfo { width: SCREEN_WIDTH, height: SCREEN_HEIGHT},
            CameraInfo { scale: CAMERA_SCALE },
            GridSize { width: GRID_WIDTH, height: GRID_HEIGHT}
        )
    }

    fn test_world_to_screen_pos(world_pos: Point, screen_pos: Point) {
        let (screen, camera, _) = setup_info();
        let result_point = world_to_screen_pos(&screen, &camera, world_pos);
        assert_eq!(result_point, screen_pos, "Expected screen position ({}, {}), got ({}, {})", screen_pos.x, screen_pos.y, result_point.x, result_point.y);
    }

    fn test_screen_to_grid_pos(screen_pos: Point, grid_pos: Point) {
        let (screen, camera, grid) = setup_info();
        let result_point = screen_to_grid_pos(&screen, &camera, &grid, screen_pos);
        assert_eq!(result_point, grid_pos, "Expected grid position ({}, {}), got ({}, {})", grid_pos.x, grid_pos.y, result_point.x, result_point.y);
    }

    #[test]
    fn world_pos_origin_to_screen_pos_center() {
        let world_pos : Point = Point::new(0, 0);
        let expected_pos : Point = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        test_world_to_screen_pos(world_pos, expected_pos);
    }

    #[test]
    fn world_pos_point_to_scaled_screen_pos() {
        let world_pos : Point = Point::new(100, 100);
        let expected_pos : Point = Point::new(SCREEN_WIDTH / 2 + 100 * CAMERA_SCALE as i32, SCREEN_HEIGHT / 2 + 100 * CAMERA_SCALE as i32);
        test_world_to_screen_pos(world_pos, expected_pos);
    }

    #[test]
    fn screen_pos_center_to_grid_pos_origin() {
        let screen_pos = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let grid_pos = Point::new(0, 0);
        test_screen_to_grid_pos(screen_pos, grid_pos);
    }

    #[test]
    fn screen_pos_negative_grid_pos() {
        let screen_pos = Point::new(SCREEN_WIDTH / 2 - 1, SCREEN_HEIGHT / 2 - 1);
        let grid_pos = Point::new(-1, -1);
        test_screen_to_grid_pos(screen_pos, grid_pos);
    }
}