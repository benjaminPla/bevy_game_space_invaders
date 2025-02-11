use crate::alien;
use crate::animations;
use crate::player;
use crate::plugins::resolution;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(resolution::ResolutionPlugin);
        app.add_systems(
            Startup,
            (setup_game, player::setup_player, alien::setup_alien),
        );
        app.add_systems(Update, animations::execute_animations);
    }
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);
}
