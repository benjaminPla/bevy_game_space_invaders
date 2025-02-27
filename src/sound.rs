use crate::game;
use bevy::prelude::*;
use bevy_kira_audio::prelude::{AudioSource, *};

#[derive(Component)]
pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(Startup, setup)
            .add_event::<SoundEvents>()
            .add_systems(PostStartup, music)
            .add_systems(Update, sounds.run_if(in_state(game::GameState::Playing)));
    }
}

#[derive(Resource)]
struct SoundResource {
    explosion: Handle<AudioSource>,
    game_over: Handle<AudioSource>,
    level_completed: Handle<AudioSource>,
    music: Handle<AudioSource>,
    projectile: Handle<AudioSource>,
}

#[derive(Event)]
pub enum SoundEvents {
    Explosion,
    GameOver,
    LevelCompleted,
    Projectile,
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let sound_resource = SoundResource {
        explosion: asset_server.load("sounds/explosion.mp3"),
        game_over: asset_server.load("sounds/game_over.mp3"),
        level_completed: asset_server.load("sounds/level_completed.mp3"),
        music: asset_server.load("sounds/music.mp3"),
        projectile: asset_server.load("sounds/projectile.mp3"),
    };
    commands.insert_resource(sound_resource);
}

fn music(audio: Res<Audio>, sound_resource: Res<SoundResource>) {
    audio.play(sound_resource.music.clone()).looped();
}

fn sounds(audio: Res<Audio>, mut events: EventReader<SoundEvents>, sounds: Res<SoundResource>) {
    for event in events.read() {
        match event {
            SoundEvents::Explosion => audio.play(sounds.explosion.clone()),
            SoundEvents::GameOver => audio.play(sounds.game_over.clone()),
            SoundEvents::LevelCompleted => audio.play(sounds.level_completed.clone()),
            SoundEvents::Projectile => audio.play(sounds.projectile.clone()),
        };
    }
}
