use crate::alien;
use crate::animations;
use crate::player;
use crate::plugins;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(plugins::resolution::ResolutionPlugin);
        app.add_plugins(plugins::alien_movement::AlienMovementPlugin);
        app.add_plugins(player::PlayerPlugin);
        app.add_plugins(alien::AlienPlugin);
        app.add_plugins(plugins::player_movement::PlayerMovementPlugin);
        app.add_plugins(plugins::player_shooting::PlayerShootingPlugin);
        app.add_systems(Startup, setup_game);
        app.add_systems(Update, animations::execute_animations);
    }
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);
}
