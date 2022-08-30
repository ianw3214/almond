mod components;
mod renderer;

use specs::prelude::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};

use crate::components::*;

struct State {
    ecs: World
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("Could not initialize image context");
    let window = video_subsystem.window("game", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()
        .expect("could not create event pump");

    let textures = [
        texture_creator.load_texture("assets/villager.png").unwrap(),
        texture_creator.load_texture("assets/villager2.png").unwrap(),
        texture_creator.load_texture("assets/tree.png").unwrap()
    ];

    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs.create_entity()
        .with(Position{ x: 40, y: 25})
        .with(Renderable{ i : 0 })
        .build();

    canvas.set_draw_color(Color::RGB(64, 64, 255));

    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, ..} => {
                    println!("{}, {}", x, y);
                    // mouse_command = Some(MouseCommand::Click(Point::new(x, y)));
                }
                _ => {}
            }
        }

        // render
        canvas.clear(); 

        renderer::render(&mut canvas, &textures, &gs.ecs);
        canvas.present();
    }
}