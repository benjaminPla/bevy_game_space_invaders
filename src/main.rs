mod animations;
mod collisions;
mod constants;
mod enemy;
mod enemy_movement;
mod game_state;
mod player;
mod player_movement;
mod projectiles;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(MainPlugin).run();
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_plugins((
            DefaultPlugins,
            collisions::CollisionsPlugin,
            game_state::GameStatePlugin,
            player::PlayerPlugin,
            projectiles::ProjectilesPlugin,
            enemy::EnemyPlugin,
            enemy_movement::EnemyMovementPlugin,
            player_movement::PlayerMovementPlugin,
        ));
        app.add_systems(Update, animations::execute_animations);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
