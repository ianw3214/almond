use sdl2::EventPump;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::{self, InitFlag};
use sdl2::ttf::Sdl2TtfContext;

use std::time::SystemTime;

pub struct Engine {
    pub canvas : WindowCanvas,
    pub event_pump : EventPump,
    pub texture_creator : TextureCreator<WindowContext>,

    pub ttf_context : Sdl2TtfContext,

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

    
    // text rendering
    let ttf_context = sdl2::ttf::init().expect("Could not initialize ttf context");

    Engine {
        canvas : canvas,
        event_pump : event_pump,
        texture_creator : texture_creator,
        ttf_context : ttf_context,
        last_update : SystemTime::now()
    }
}