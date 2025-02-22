use crate::game;
use bevy::prelude::*;

#[derive(Component)]
pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<SoundEvents>()
            .add_systems(Update, sounds.run_if(in_state(game::GameState::Playing)));
    }
}

#[derive(Event)]
pub enum SoundEvents {
    Explosion,
    GameOver,
    LevelCompleted,
    Projectile,
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/background.mp3")),
        PlaybackSettings::LOOP,
    ));
}

fn sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SoundEvents>,
) {
    for event in events.read() {
        match event {
            SoundEvents::Explosion => {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/explosion.mp3")),
                    PlaybackSettings::ONCE,
                ));
            }
            SoundEvents::GameOver => {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/game_over.mp3")),
                    PlaybackSettings::ONCE,
                ));
            }
            SoundEvents::LevelCompleted => {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/level_completed.mp3")),
                    PlaybackSettings::ONCE,
                ));
            }
            SoundEvents::Projectile => {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/projectile.mp3")),
                    PlaybackSettings::ONCE,
                ));
            }
        }
    }
}
