mod animations;
mod background;
mod collisions;
mod constants;
mod controls;
mod enemy;
mod enemy_movement;
mod game;
mod player;
mod projectiles;
mod sound;
mod sprites;
mod texts;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn main() {
    App::new().add_plugins(MainPlugin).run();
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, AudioPlugin))
            .init_state::<game::GameState>()
            .add_systems(PreStartup, setup)
            .add_plugins((
                animations::AnimationPlugin,
                background::BackgroundPlugin,
                collisions::CollisionsPlugin,
                controls::ControlsPlugin,
                enemy::EnemyPlugin,
                enemy_movement::EnemyMovementPlugin,
                game::GamePlugin,
                player::PlayerPlugin,
                projectiles::ProjectilesPlugin,
                sound::SoundPlugin,
                sprites::SpritesPlugin,
                texts::TextsPlugin,
            ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(ClearColor(Color::srgb(0., 0., 0.)));
}
