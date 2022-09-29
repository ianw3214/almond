use sdl2::EventPump;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::{self, InitFlag};

use std::time::SystemTime;

use crate::engine::text::*;

pub struct Engine {
    pub canvas : WindowCanvas,
    pub event_pump : EventPump,
    pub texture_creator : TextureCreator<WindowContext>,

    pub text : TextEngine,

    pub last_update : SystemTime
}

pub fn init_engine() -> Engine {
    // Initialize SDL and related subsystems
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("Could not initialize image context");
    let window = video_subsystem.window("game", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_context.event_pump()
        .expect("could not create event pump");

    let text = TextEngine::new();

    Engine {
        canvas : canvas,
        event_pump : event_pump,
        texture_creator : texture_creator,
        text : text,
        last_update : SystemTime::now()
    }
}