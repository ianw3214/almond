mod components;
mod renderer;
mod map;
mod brain;
mod pathfinder;
mod scheduler;

use specs::prelude::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};

use crate::components::*;
use crate::map::*;

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
        texture_creator.load_texture("assets/grass.png").unwrap(),
        texture_creator.load_texture("assets/tree.png").unwrap(),
        texture_creator.load_texture("assets/flint.png").unwrap(),
        texture_creator.load_texture("assets/water.png").unwrap(),
        texture_creator.load_texture("assets/storage.png").unwrap(),
        texture_creator.load_texture("assets/house.png").unwrap()
    ];

    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Animatable>();
    gs.ecs.register::<ResourceSource>();
    gs.ecs.register::<Brain>();
    gs.ecs.register::<Inventory>();
    gs.ecs.register::<Movement>();

    let mut dispatcher = DispatcherBuilder::new()
    .with(scheduler::Scheduler, "Scheduler", &[])
        .with(brain::AI, "AI", &["Scheduler"])
        .with(pathfinder::Pathfinder, "Pathfinder", &["AI"])
        .build();
    dispatcher.setup(&mut gs.ecs);

    let _npc = gs.ecs.create_entity()
        .with(Position{ x: 40, y: 25})
        .with(Renderable{ i : 0 })
        .with(Animatable{ width: 30, height: 40, frame: 0 })
        .with(Brain{ task : Task::IDLE })
        .with(Inventory{ resources: vec![ (ResourceType::WOOD, 0), (ResourceType::FLINT, 0)]})
        .with(Movement{ speed : 1, target: None })
        .build();
    
    let wood = gs.ecs.create_entity()
        .with(Position{ x: 100, y: 100})
        .with(Renderable{ i : 2})
        .with(ResourceSource{ amount: 10, resource_type: ResourceType::WOOD})
        .build();

    let flint = gs.ecs.create_entity()
        .with(Position {x: 200, y: 200})
        .with(Renderable{ i : 3})
        .with(ResourceSource{ amount: 10, resource_type: ResourceType::FLINT})
        .build();

    let store = gs.ecs.create_entity()
        .with(Position {x: 300, y: 300})
        .with(Renderable{ i : 5})
        .with(ResourceStorage{ resources:vec![ (ResourceType::WOOD, 0), (ResourceType::FLINT, 0)], max: 10})
        .build();

    // global resources
    gs.ecs.insert(new_map());
    gs.ecs.insert(vec![ Task::STORE(Some(store)), Task::COLLECT(Some(wood)), Task::COLLECT(Some(flint))]);

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
                    let building = gs.ecs.create_entity()
                        .with(Position{ x : x, y : y })
                        .with(Renderable{ i : 6 })
                        .with(Construction{ counter : 0 })
                        .build();
                    // Add a task to construct the building
                    let mut taskqueue = gs.ecs.write_resource::<Vec<Task>>();
                    taskqueue.push(Task::BUILD(Some(building)));
                }
                _ => {}
            }
        }

        // update world
        dispatcher.dispatch(&mut gs.ecs);
        gs.ecs.maintain();

        // render
        canvas.clear(); 

        render_map(&gs.ecs.fetch::<Vec<TileType>>(), &mut canvas, &textures);
        
        renderer::render(&mut canvas, &textures, &gs.ecs);
        canvas.present();
    }
}