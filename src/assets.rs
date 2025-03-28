use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::{AudioSource, *};

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/explosion.mp3")]
    pub explosion: Handle<AudioSource>,
    #[asset(path = "audio/game_over.mp3")]
    pub game_over: Handle<AudioSource>,
    #[asset(path = "audio/level_completed.mp3")]
    pub level_completed: Handle<AudioSource>,
    #[asset(path = "audio/music.mp3")]
    pub music: Handle<AudioSource>,
    #[asset(path = "audio/projectile.mp3")]
    pub projectile: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/font.ttf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/enemy.png")]
    pub enemy: Handle<Image>,
    #[asset(path = "sprites/planet.png")]
    pub planet: Handle<Image>,
    #[asset(path = "sprites/player.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/projectile.png")]
    pub projectile: Handle<Image>,
    #[asset(path = "sprites/star_big.png")]
    pub star_big: Handle<Image>,
    #[asset(path = "sprites/star_small.png")]
    pub star_small: Handle<Image>,
}

pub fn start_background_audio(audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
    audio.play(audio_assets.music.clone()).looped();
}
