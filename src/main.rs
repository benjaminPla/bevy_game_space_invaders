mod animations;
mod collisions;
mod constants;
mod enemy;
mod enemy_movement;
mod game;
mod player;
mod controls;
mod projectiles;
mod texts;

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
                controls::ControlsPlugin,
                enemy::EnemyPlugin,
                enemy_movement::EnemyMovementPlugin,
                game::GamePlugin,
                player::PlayerPlugin,
                projectiles::ProjectilesPlugin,
                texts::TextsPlugin,
            ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
