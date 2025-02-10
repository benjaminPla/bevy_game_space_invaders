use crate::alien;
use crate::animations;
use crate::player;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_game, alien::setup_alien));
        app.add_systems(Update, animations::execute_animations);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        player::Player::new(),
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(100., 0., 0.),
    ));
}
