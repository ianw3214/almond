mod components;
mod physics;
mod animator;
mod input;
mod renderer;
mod grid;
mod ai;

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

pub struct GridSize {
    width : i32,
    height : i32
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
    match direction {
        Direction::Up => 3,
        Direction::Down => 0,
        Direction::Left => 1,
        Direction::Right => 2
    }
}

fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height
            )
        });
    }
    frames
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
        .with(grid::Grid, "Grid", &[])
        .with(input::keyboard::Keyboard, "Keyboard", &[])
        .with(input::mouse::Mouse, "Mouse", &[])
        .with(ai::AI, "AI", &[])
        .with(physics::Physics, "Physics", &["Keyboard", "AI"])
        .with(animator::Animator, "Animator", &["Keyboard", "AI"])
        .build();
    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command : Option<MovementCommand> = None;
    world.insert(movement_command);
    let mouse_command : Option<MouseCommand> = None;
    world.insert(mouse_command);

    let grid_size : GridSize = GridSize { width : 40, height : 40};
    world.insert(grid_size);
    
    let textures = [
        texture_creator.load_texture("assets/villager.png")?,
        texture_creator.load_texture("assets/tree.png")?
    ];
    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 30, 40);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right)
    };

    let ai_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right)
    };

    // Player
    world.create_entity()
        .with(KeyboardControlled)
        .with(WorldPosition(Point::new(0, 0)))
        .with(Velocity{speed:0, direction: Direction::Right})
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    // Tree
    world.create_entity()
        .with(WorldPosition(Point::new(0, 0)))
        .with(GridPosition{ x : 5, y : -5})
        .with(Sprite{ spritesheet : 1, region : Rect::new(0, 0, 40, 60)})
        .with(Clickable)
        .build();

    // AI
    world.create_entity()
        .with(Brain)
        .with(WorldPosition(Point::new(0, 0)))
        .with(Velocity{speed:0, direction: Direction::Right})
        .with(ai_animation.right_frames[0].clone())
        .with(ai_animation)
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
        renderer::render(&mut canvas, Color::RGB(64, 64, 255), &textures, world.system_data())?;
        ::std::thread::sleep(Duration::new(0, 1_100_000_000u32 / 20));
    }

    Ok(())
}
