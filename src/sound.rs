use crate::game;
use bevy::prelude::*;

#[derive(Component)]
pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<SoundEvents>()
            .add_systems(
                Update,
                projectile.run_if(in_state(game::GameState::Playing)),
            );
    }
}

#[derive(Event)]
pub enum SoundEvents {
    Projectile,
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/background.mp3")),
        PlaybackSettings::LOOP,
    ));
}

fn projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SoundEvents>,
) {
    for event in events.read() {
        match event {
            SoundEvents::Projectile => {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/projectile.mp3")),
                    PlaybackSettings::ONCE,
                ));
            }
        }
    }
}
