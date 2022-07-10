mod components;
mod util;
mod systems;
mod input;
mod ui;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::prelude::*;

use std::time::Duration;

use crate::components::*;

pub enum MovementCommand {
    Stop,
    Move(Direction)
}

pub enum MouseCommand {
    Click(Point)
} 

pub struct ScreenInfo {
    width : i32,
    height : i32
}

pub struct GridSize {
    width : i32,
    height : i32
}

pub struct CameraInfo {
    scale : f32
}

fn get_screen_info(canvas: &sdl2::render::WindowCanvas) -> ScreenInfo {
    let (w, h) = canvas.output_size().expect("canvas should give an output size");
    ScreenInfo {
        width : w as i32,
        height : h as i32
    }
}

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("game", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()
        .expect("could not create event pump");
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::grid::Grid, "Grid", &[])
        .with(input::mouse::action::Action, "Action", &[])
        .with(input::mouse::clickable::Clickable, "Clickable", &["Action"])
        .with(systems::animator::Animator, "Animator", &[])
        .with(systems::turn::TurnSystem, "Turn", &[])
        .build();
    let mut world = World::new();
    dispatcher.setup(&mut world);
    systems::renderer::SystemData::setup(&mut world);
    ui::renderer::SystemData::setup(&mut world);

    // insert global resources
    let movement_command : Option<MovementCommand> = None;
    world.insert(movement_command);
    let mouse_command : Option<MouseCommand> = None;
    world.insert(mouse_command);
    let grid_size : GridSize = GridSize { width : 40, height : 40};
    world.insert(grid_size);
    let screen_info : ScreenInfo = get_screen_info(&canvas);
    world.insert(screen_info);
    let camera_info : CameraInfo = CameraInfo { scale: 2.0 };
    world.insert(camera_info);

    let textures = [
        texture_creator.load_texture("assets/villager.png")?,
        texture_creator.load_texture("assets/tree.png")?
    ];
    let ui_textures = [
        texture_creator.load_texture("assets/selected.png")?
    ];
    let player_animation = Animation {
        current_frame: 0,
        current_anim: 0,
        animations: vec![(0, 3)]
    };

    let ai_animation = Animation {
        current_frame: 0,
        current_anim: 0,
        animations: vec![(0, 3)]
    };

    // Player
    world.create_entity()
        .with(WorldPosition(Point::new(0, 0)))
        .with(GridPosition{ x: 0, y: 0 })
        .with(Sprite { spritesheet: 0, region: Rect::new(0, 0, 30, 40)})
        .with(player_animation)
        .with(Selectable{ width: 30, height: 40, selected: false })
        .with(Turn{ current: false })
        .build();

    // Tree
    world.create_entity()
        .with(WorldPosition(Point::new(0, 0)))
        .with(GridPosition{ x: 2, y: -2})
        .with(Sprite{ spritesheet: 1, region: Rect::new(0, 0, 40, 60)})
        .with(Selectable{ width: 40, height: 60, selected: false })
        .build();

    // AI
    world.create_entity()
        // .with(Brain)
        .with(WorldPosition(Point::new(0, 0)))
        .with(GridPosition{ x: -2, y: 1 })
        .with(Sprite { spritesheet: 0, region: Rect::new(0, 0, 30, 40)})
        .with(ai_animation)
        .with(Selectable{ width: 40, height: 60, selected: false })
        .with(Turn{ current: false })
        .build();

    'running: loop {
        let mut movement_command = None;
        let mut mouse_command = None;

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, ..} => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, ..} => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, ..} => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, ..} => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, ..} => {
                    movement_command = Some(MovementCommand::Stop);
                },
                Event::MouseButtonDown { x, y, ..} => {
                    mouse_command = Some(MouseCommand::Click(Point::new(x, y)));
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;
        *world.write_resource() = mouse_command;

        dispatcher.dispatch(&mut world);
        world.maintain();

        // render
        canvas.set_draw_color(Color::RGB(64, 64, 255));
        canvas.clear(); 

        systems::renderer::render(&mut canvas, &textures, world.system_data())?;
        ui::renderer::render(&mut canvas, &ui_textures, world.system_data())?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_100_000_000u32 / 20));
    }

    Ok(())
}
