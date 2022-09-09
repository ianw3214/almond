use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture};

use rand::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Grass, Water
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Grass; 80 * 50];
    let mut rng = thread_rng();
    for _ in 0..100 {
        let index = rng.gen_range(0..80 * 50);
        map[index] = TileType::Water;
    }
    map
}

pub fn render_map(map : &Vec<TileType>, canvas : &mut WindowCanvas, textures : &[Texture]) {
    // draw the map
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending on the type
        match tile {
            TileType::Grass => {
                let screen_rect = Rect::from_center(Point::new(x * 16, y * 16), 16, 16);
                canvas.copy(&textures[1], None, screen_rect).expect("render copy failed...");
            },
            TileType::Water => {
                let screen_rect = Rect::from_center(Point::new(x * 16, y * 16), 16, 16);
                canvas.copy(&textures[4], None, screen_rect).expect("render copy failed...");
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}