mod components;
mod util;
mod systems;
mod input;
mod ui;
mod gameplay;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, InitFlag};

use specs::prelude::*;

use std::collections::VecDeque;

use std::time::Instant;

use crate::components::*;
use crate::gameplay::action::*;

#[derive(Clone)]
pub enum MouseCommand {
    Click(Point)
} 

pub struct ScreenInfo {
    width : i32,
    height : i32
}

pub struct MouseInfo {
    x : i32,
    y : i32
}

pub struct GridSize {
    width : i32,
    height : i32
}

pub struct CameraInfo {
    scale : f32
}

pub enum UIAction {
    ActionButton(i32)
}

#[derive(Debug)]
pub enum CurrentAction {
    None,
    Action(i32)
}

pub struct SelectedEntity(Option<Entity>);

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
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::grid::Grid, "Grid", &[])
        .with(input::mouse::Mouse, "Action", &[])
        .with(systems::animator::Animator, "Animator", &[])
        .with(systems::turn::TurnSystem{ last_prio: std::i32::MIN }, "Turn", &[])
        .build();
    let mut world = World::new();
    dispatcher.setup(&mut world);
    systems::renderer::SystemData::setup(&mut world);
    ui::renderer::SystemData::setup(&mut world);
    ui::hud::SystemData::setup(&mut world);

    // insert global resources
    let mouse_command : Option<MouseCommand> = None;
    world.insert(mouse_command);
    let grid_size : GridSize = GridSize { width : 40, height : 40};
    world.insert(grid_size);
    let screen_info : ScreenInfo = get_screen_info(&canvas);
    world.insert(screen_info);
    let camera_info : CameraInfo = CameraInfo { scale: 2.0 };
    world.insert(camera_info);
    let mouse_info : MouseInfo = MouseInfo { x: 0, y: 0 };
    world.insert(mouse_info);
    let ui_commands : VecDeque<UIAction> = VecDeque::new();
    world.insert(ui_commands);
    let current_action : CurrentAction = CurrentAction::None;
    world.insert(current_action);
    let selected_entity : SelectedEntity = SelectedEntity(None);
    world.insert(selected_entity);

    let textures = [
        texture_creator.load_texture("assets/villager.png")?,
        texture_creator.load_texture("assets/villager2.png")?,
        texture_creator.load_texture("assets/tree.png")?
    ];
    let ui_textures = [
        texture_creator.load_texture("assets/selected.png")?,
        texture_creator.load_texture("assets/grid_hover.png")?,
        texture_creator.load_texture("assets/turn.png")?
    ];
    let mut hud_textures = [
        texture_creator.load_texture("assets/move_icon.png")?,
        texture_creator.load_texture("assets/heart.png")?,
        texture_creator.load_texture("assets/heart_empty.png")?,
        texture_creator.load_texture("assets/attack_icon.png")?
    ];
    let player_animation = Animation {
        current_frame: 0,
        current_anim: 0,
        animations: vec![(0, 3)],
        last_update: Instant::now()
    };

    let ai_animation = Animation {
        current_frame: 0,
        current_anim: 0,
        animations: vec![(0, 3)],
        last_update: Instant::now()
    };

    // Player
    world.create_entity()
        .with(WorldPosition{ point: Point::new(0, 0)})
        .with(GridPosition{ x: 0, y: 0 })
        .with(Sprite { spritesheet: 0, region: Rect::new(0, 0, 30, 40), x_offset: -15, y_offset: -40})
        .with(player_animation)
        .with(Selectable{ width: 30, height: 40, x_offset: -15, y_offset: -40 })
        .with(Turn{ current: false, priority: 1, actions: vec![ Action{ range: 5, effects: vec![ActionEffect::Move] }, Action{ range: 4, effects: vec![ActionEffect::Damage(1)] } ] })
        .with(Health{ health: 5, max_health: 5})
        .build();

    // Player 2
    world.create_entity()
        // .with(Brain)
        .with(WorldPosition{ point: Point::new(0, 0) })
        .with(GridPosition{ x: 1, y: 1 })
        .with(Sprite { spritesheet: 1, region: Rect::new(0, 0, 30, 40), x_offset: -15, y_offset: -40})
        .with(ai_animation)
        .with(Selectable{ width: 30, height: 40, x_offset: -15, y_offset: -40 })
        .with(Turn{ current: false, priority: 2, actions: vec![ Action{ range:5, effects: vec![ActionEffect::Move] }, Action{ range:10, effects: vec![ActionEffect::Damage(2)] } ]  })
        .with(Health{ health: 5, max_health: 5})
        .build();

    // Tree
    world.create_entity()
        .with(WorldPosition{ point: Point::new(0, 0) })
        .with(GridPosition{ x: 1, y: -1})
        .with(Sprite{ spritesheet: 2, region: Rect::new(0, 0, 40, 60), x_offset: -20, y_offset: -60})
        .with(Selectable{ width: 40, height: 60, x_offset: -25, y_offset: -60 })
        .build();

    'running: loop {
        let mut mouse_command = None;

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                /*
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
                 */
                Event::MouseButtonDown { x, y, ..} => {
                    mouse_command = Some(MouseCommand::Click(Point::new(x, y)));
                }
                _ => {}
            }
        }

        let mouse = event_pump.mouse_state();
        let mouse_info : MouseInfo = MouseInfo { x : mouse.x(), y : mouse.y() };
        
        *world.write_resource() = mouse_info;
        *world.write_resource() = mouse_command;

        // update custom systems
        ui::hud::run(world.system_data());

        dispatcher.dispatch(&mut world);
        world.maintain();

        // render
        canvas.set_draw_color(Color::RGB(64, 64, 255));
        canvas.clear(); 

        systems::renderer::render(&mut canvas, &textures, world.system_data())?;
        ui::renderer::render(&mut canvas, &ui_textures, world.system_data())?;
        ui::hud::render(&mut canvas, &mut hud_textures, world.system_data())?;

        canvas.present();
    }

    Ok(())
}
