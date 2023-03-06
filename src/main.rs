mod game;
mod input;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(game::game::Game)
        .run();
}
