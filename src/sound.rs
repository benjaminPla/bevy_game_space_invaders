use crate::assets;
use crate::game;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
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

fn sounds(
    audio: Res<Audio>,
    mut events: EventReader<SoundEvents>,
    audio_assets: Res<assets::AudioAssets>,
) {
    for event in events.read() {
        match event {
            SoundEvents::Explosion => audio.play(audio_assets.explosion.clone()),
            SoundEvents::GameOver => audio.play(audio_assets.game_over.clone()),
            SoundEvents::LevelCompleted => audio.play(audio_assets.level_completed.clone()),
            SoundEvents::Projectile => audio.play(audio_assets.projectile.clone()),
        };
    }
}
