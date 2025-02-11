mod alien;
mod animations;
mod player;
mod plugins;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::game::GamePlugin))
        .run();
}
