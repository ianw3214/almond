mod game;
mod input;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(game::game::Game)
        .add_plugin(DebugLinesPlugin::default())
        .run();
}
