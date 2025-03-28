mod animations;
mod assets;
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
mod texts;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

fn main() {
    App::new().add_plugins(MainPlugin).run();
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .init_state::<game::GameState>()
            .add_systems(Startup, setup)
            .add_loading_state(
                LoadingState::new(game::GameState::Loading)
                    .load_collection::<assets::AudioAssets>()
                    .load_collection::<assets::FontAssets>()
                    .load_collection::<assets::SpriteAssets>()
                    .continue_to_state(game::GameState::Playing),
            )
            // move this to a plugin
            .add_systems(
                OnEnter(game::GameState::Playing),
                assets::start_background_audio,
            )
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
                texts::TextsPlugin,
            ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(ClearColor(Color::srgb(0., 0., 0.)));
}
