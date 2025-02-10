mod alien;
mod animations;
mod controlls;
mod player;
mod plugins;

use bevy::prelude::*;
use bevy_trickfilm::Animation2DPlugin;

fn main() {
    App::new()
        .add_plugins((
            (DefaultPlugins, Animation2DPlugin),
            plugins::game::GamePlugin,
        ))
        .add_systems(Update, (controlls::player_input, player::player_movement))
        .run();
}
