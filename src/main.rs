mod animations;
mod collisions;
mod constants;
mod enemy;
mod enemy_movement;
mod game;
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
        app.add_plugins(DefaultPlugins)
            .init_state::<game::GameState>()
            .add_systems(Startup, setup)
            .add_plugins((
                animations::AnimationPlugin,
                collisions::CollisionsPlugin,
                game::GamePlugin,
                player::PlayerPlugin,
                projectiles::ProjectilesPlugin,
                enemy::EnemyPlugin,
                enemy_movement::EnemyMovementPlugin,
                player_movement::PlayerMovementPlugin,
            ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
