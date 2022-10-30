use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture};
use specs::{World, WorldExt};

use crate::CameraInfo;

// use rand::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Grass, Water
}

/////////////////////////////////////////////////
const PERLIN_GRID_SIZE : i32 = 10;
const TABLE_SIZE : usize = 256;
const VALUES : [u8; TABLE_SIZE] = [
    151,160,137,91,90,15,                 
    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
];
const ROOT_TWO : f32 = 1.41421356;
const GRAD_TABLE_SIZE : usize = 4;
const GRAD_TABLE : [Vector; GRAD_TABLE_SIZE] = [
    Vector{ x : ROOT_TWO, y :  ROOT_TWO},
    Vector{ x : ROOT_TWO, y : -ROOT_TWO},
    Vector{ x : -ROOT_TWO, y :  ROOT_TWO},
    Vector{ x : -ROOT_TWO, y : -ROOT_TWO}
];

#[derive(Clone, Copy)]
struct Vector {
    x : f32,
    y : f32
}

fn grad(x : i32, y : i32) -> Vector {
    let first = (x & 0xff) as usize;
    let second = (y & 0xff) as usize;

    let index = VALUES[first] as usize ^ second;
    let hash = VALUES[index];

    let grad_index = (hash % GRAD_TABLE_SIZE as u8) as usize;
    GRAD_TABLE[grad_index]
}

fn dot(v1 : Vector, v2 : Vector) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

// this returns a value between 0 and 1
fn smoothstep(num : i32) -> f32 {
    let t = (num as f32) / (PERLIN_GRID_SIZE as f32);
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a : f32, b : f32, val : f32) -> f32 {
    a + (b - a) * val
}

fn perlin_noise(x : i32, y : i32) -> f32 {
    let left = (x / PERLIN_GRID_SIZE) * PERLIN_GRID_SIZE;
    let top = (y / PERLIN_GRID_SIZE) * PERLIN_GRID_SIZE;

    let grad_top_left = grad(left, top);
    let grad_top_right = grad(left + PERLIN_GRID_SIZE, top);
    let grad_bottom_left = grad(left, top + PERLIN_GRID_SIZE);
    let grad_bottom_right = grad(left + PERLIN_GRID_SIZE, top + PERLIN_GRID_SIZE);

    let dist_left = (x - left) as f32;
    let dist_right = (left + PERLIN_GRID_SIZE - x) as f32;
    let dist_top = (y - top) as f32;
    let dist_bottom = (top + PERLIN_GRID_SIZE - y) as f32;
    let dist_top_left = Vector{x : dist_left, y : dist_top };
    let dist_top_right = Vector{x : dist_right, y : dist_top };
    let dist_bot_left = Vector{x : dist_left, y : dist_bottom };
    let dist_bot_right = Vector{x : dist_right, y : dist_bottom };

    let dot_top_left = dot(grad_top_left, dist_top_left);
    let dot_top_right = dot(grad_top_right, dist_top_right);
    let dot_bot_left = dot(grad_bottom_left, dist_bot_left);
    let dot_bot_right = dot(grad_bottom_right, dist_bot_right);

    let smooth_x = smoothstep(x - left);
    let smooth_y = smoothstep(y - top);

    let nx0 = lerp(dot_top_left, dot_top_right, smooth_x);
    let nx1 = lerp(dot_bot_left, dot_bot_right, smooth_x);

    lerp(nx0, nx1, smooth_y)
}
////////////////////////////////////////////////////

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

const MAP_WIDTH : usize = 80;
const MAP_HEIGHT : usize = 50;
pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Grass; MAP_WIDTH * MAP_HEIGHT];
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let val = perlin_noise(x as i32, y as i32);
            if val < 0.0 {
                let index = y * MAP_WIDTH + x;
                map[index] = TileType::Water;
            }
        }
    }
    map
}

pub fn render_map(map : &Vec<TileType>, canvas : &mut WindowCanvas, textures : &Vec<Texture>, world: &World) {
    let cameraInfo = world.read_resource::<CameraInfo>();
    
    // draw the map
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        let screen_x = x * 16 - cameraInfo.x as i32;
        let screen_y = y * 16 - cameraInfo.y as i32;
        // Render a tile depending on the type
        match tile {
            TileType::Grass => {
                let screen_rect = Rect::from_center(Point::new(screen_x, screen_y), 16, 16);
                canvas.copy(&textures[1], None, screen_rect).expect("render copy failed...");
            },
            TileType::Water => {
                let screen_rect = Rect::from_center(Point::new(screen_x, screen_y), 16, 16);
                canvas.copy(&textures[4], None, screen_rect).expect("render copy failed...");
            }
        }
        x += 1;
        if x >= MAP_WIDTH as i32 {
            x = 0;
            y += 1;
        }
    }
}