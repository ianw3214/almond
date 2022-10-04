mod engine;
mod components;
mod renderer;
mod map;
mod brain;
mod pathfinder;
mod scheduler;
mod hud;
mod debug;
mod gameplay;

use specs::prelude::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::components::*;
use crate::map::*;

use hud::UIEvent;

use std::time::SystemTime;

enum CursorState {
    DEFAULT,
    BUILD,
    COLLECT
}

#[derive(Default)]
pub struct DeltaTime(f32);

struct State {
    ecs: World
}

pub struct TownInfo {
    pub name : String
}

fn main() {

    let mut engine = engine::engine::init_engine();
    let mut textures = engine::resource::TextureManager::new(&engine.texture_creator);
    textures.load("assets/villager.png");
    textures.load("assets/grass.png");
    textures.load("assets/tree.png");
    textures.load("assets/flint.png");
    textures.load("assets/water.png");
    textures.load("assets/storage.png");
    textures.load("assets/house.png");
    textures.load("assets/banner.png");

    let mut ui_textures = engine::resource::TextureManager::new(&engine.texture_creator);
    ui_textures.load("assets/ui/background.png");
    ui_textures.load("assets/ui/build.png");
    ui_textures.load("assets/ui/collect.png");
    ui_textures.load("assets/ui/progress_bar.png");
    ui_textures.load("assets/ui/progress_bar_bg.png");

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
    gs.ecs.register::<BoundingBox>();
    gs.ecs.register::<TownCenter>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(scheduler::Scheduler, "Scheduler", &[])
        .with(brain::AI, "AI", &["Scheduler"])
        .with(pathfinder::Pathfinder, "Pathfinder", &["AI"])
        .with(gameplay::housing::HousingSystem, "Housing", &["AI"])
        .build();
    dispatcher.setup(&mut gs.ecs);

    let _npc = gs.ecs.create_entity()
        .with(Position{ x: 40, y: 25})
        .with(Renderable{ i : 0 })
        .with(Animatable{ width: 30, height: 40, frame: 0, timer: 0.0 })
        .with(Brain{ task : Task::IDLE })
        .with(Inventory{ resources: vec![ (ResourceType::WOOD, 0), (ResourceType::FLINT, 0)]})
        .with(Movement{ speed : 1, target: None })
        .with(BoundingBox{ width : 30, height : 40, x_offset : 0, y_offset : 0 })
        .with(Tenant{ house : None })
        .build();
    
    let _wood = gs.ecs.create_entity()
        .with(Position{ x: 100, y: 100})
        .with(Renderable{ i : 2})
        .with(ResourceSource{ amount: 10, resource_type: ResourceType::WOOD})
        .with(BoundingBox{ width : 40, height : 40, x_offset : 0, y_offset : 0 })
        .build();

    let _flint = gs.ecs.create_entity()
        .with(Position {x: 200, y: 200})
        .with(Renderable{ i : 3})
        .with(ResourceSource{ amount: 10, resource_type: ResourceType::FLINT})
        .with(BoundingBox{ width : 40, height : 40, x_offset : 0, y_offset : 0 })
        .build();

    let _store = gs.ecs.create_entity()
        .with(Position {x: 300, y: 300})
        .with(Renderable{ i : 5})
        .with(ResourceStorage{ resources:vec![ (ResourceType::WOOD, 0), (ResourceType::FLINT, 0)], max: 10})
        .with(BoundingBox{ width : 40, height : 40, x_offset : 0, y_offset : 0 })
        .build();

    let _banner = gs.ecs.create_entity()
        .with(Position {x: 400, y: 400})
        .with(Renderable{ i : 7})
        .with(TownCenter)
        .with(BoundingBox{ width : 40, height : 40, x_offset : 0, y_offset : 0 })
        .build();

    // global resources
    gs.ecs.insert(DeltaTime(33.3));
    gs.ecs.insert(new_map());
    gs.ecs.insert(TownInfo{ name : String::from("Test Town") });
    let taskqueue : Vec<Task> = Vec::new();
    gs.ecs.insert(taskqueue);
    let eventqueue : Vec<hud::UIEvent> = Vec::new();
    gs.ecs.insert(eventqueue);
    
    // This could eventually be a global resource?
    let mut cursor_state : CursorState = CursorState::DEFAULT;

    engine.canvas.set_draw_color(Color::RGB(64, 64, 255));

    let mut ui_hud = hud::Hud::new();
    ui_hud.init();

    'running: loop {
        // delta time
        let curr = SystemTime::now();
        let delta = curr.duration_since(engine.last_update).expect("Time went backwards...");
        gs.ecs.insert(DeltaTime(delta.as_secs_f32()));
        engine.last_update = curr;

        // handle events
        for event in engine.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::MouseMotion { x, y, ..} => {
                    ui_hud.handle_mouse_motion(x, y);
                },
                Event::MouseButtonDown { x, y, ..} => {
                    let handled = ui_hud.handle_mouse_click(x, y, &mut gs.ecs);
                    if !handled {
                        // TODO: Move this into a system?
                        match cursor_state {
                            CursorState::DEFAULT => {
                                // do nothing...
                            },
                            CursorState::BUILD => {
                                let building = gs.ecs.create_entity()
                                    .with(Position{ x : x, y : y })
                                    .with(Renderable{ i : 6 })
                                    .with(Construction{ timer : 10.0 })
                                    .with(BoundingBox{ width : 40, height : 40, x_offset : 0, y_offset : 0 })
                                    .with(Housing{ capacity : 2, num_tenants : 0 })
                                    .build();
                                // Add a task to construct the building
                                let mut taskqueue = gs.ecs.write_resource::<Vec<Task>>();
                                taskqueue.push(Task::BUILD(building));
                                // reset the cursor state
                                cursor_state = CursorState::DEFAULT;
                            },
                            CursorState::COLLECT => {
                                let entities = gs.ecs.entities();
                                let positions = gs.ecs.read_storage::<Position>();
                                let resources = gs.ecs.read_storage::<ResourceSource>();
                                let aabbs = gs.ecs.read_storage::<BoundingBox>();
                                // TODO: collision box data?
                                for (entity, pos, aabb, _) in (&entities, &positions, &aabbs, &resources).join() {
                                    let pos_x = pos.x + aabb.x_offset;
                                    let pos_y = pos.y + aabb.y_offset;
                                    if x > pos_x && x < pos_x + aabb.width as i32 {
                                        if y > pos_y && y < pos_y + aabb.height as i32 {
                                            let mut taskqueue = gs.ecs.write_resource::<Vec<Task>>();
                                            taskqueue.push(Task::COLLECT(entity));
                                        }
                                    }
                                }
                                // reset the cursor state
                                cursor_state = CursorState::DEFAULT;
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // handle UI events
        {
            let mut eventqueue = gs.ecs.write_resource::<Vec<UIEvent>>();
            let event = eventqueue.last();
            if let Some(event) = event {
                match event {
                    UIEvent::Build => {
                        cursor_state = CursorState::BUILD;
                        eventqueue.pop();
                    },
                    UIEvent::Collect => {
                        cursor_state = CursorState::COLLECT;
                        eventqueue.pop();
                    }
                }
            }
        }

        // update world
        dispatcher.dispatch(&mut gs.ecs);
        gs.ecs.maintain();

        // render
        engine.canvas.clear(); 

        render_map(&gs.ecs.fetch::<Vec<TileType>>(), &mut engine.canvas, &textures.textures);
        
        debug::renderer::render(&mut engine.canvas, &gs.ecs);

        renderer::render(&mut engine.canvas, &textures.textures, &gs.ecs);
        ui_hud.render(&mut engine.canvas, &mut ui_textures, &engine.text, &gs.ecs);
        engine.canvas.present();
    }
}