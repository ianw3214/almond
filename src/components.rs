use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, PartialEq)]
pub enum ResourceType {
    WOOD,
    FLINT
}

// #[derive(Debug, PartialEq, Clone, Copy)]
#[derive(PartialEq, Clone, Copy)]
pub enum Task {
    COLLECT(Entity),
    STORE(Entity),
    BUILD(Entity),
    IDLE
}

#[derive(Component)]
pub struct Position {
    pub x : i32,
    pub y : i32
}

#[derive(Component)]
pub struct Renderable {
    pub i : usize
}

#[derive(Component)]
pub struct Animatable {
    // image data
    pub width : i32,
    pub height : i32,
    // game data
    pub frame : i32,
    pub timer : f32
}

#[derive(Component)]
pub struct ResourceSource {
    pub amount : i32,
    pub resource_type : ResourceType
}

#[derive(Component)]
pub struct ResourceStorage {
    pub resources : Vec<(ResourceType, i32)>,
    pub max : i32
}

#[derive(Component)]
pub struct Brain {
    pub task: Task
}

#[derive(Component)]
pub struct Inventory {
    pub resources : Vec<(ResourceType, i32)>
}

#[derive(Component)]
pub struct Movement {
    pub speed : i32,
    // game data
    pub target : Option<(i32, i32)>
}

// TODO: This should be based on time and resources?
#[derive(Component)]
pub struct Construction {
    pub timer : f32
}

#[derive(Component)]
pub struct BoundingBox {
    pub width : u32,
    pub height : u32,
    pub x_offset : i32,
    pub y_offset : i32
}