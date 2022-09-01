use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];
    // generate map here..
    map
}

pub fn render_map(map : &Vec<TileType>, canvas : &mut WindowCanvas, textures : &[Texture]) {
    // draw the map
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending on the type
        match tile {
            TileType::Floor => {
                let screen_rect = Rect::from_center(Point::new(x * 16, y * 16), 16, 16);
                canvas.copy(&textures[1], None, screen_rect).expect("render copy failed...");
            },
            TileType::Wall => {

            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}