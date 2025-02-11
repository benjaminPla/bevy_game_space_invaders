use crate::animations;
use crate::plugins::resolution;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

#[derive(Component)]
pub struct Player {
    position: Vec2,
    speed: f32,
    pub direction: Vec2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            speed: 200.0,
            direction: Vec2::ZERO,
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    resolution: Res<resolution::Resolution>,
) {
    let texture = asset_server.load("player.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config = animations::AnimationConfig::new(0, 1, 10);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.get_first_sprite_index(),
            }),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player::new(),
        animation_config,
    ));
}

pub fn player_movement(time: Res<Time>, mut query: Query<(&mut Transform, &mut Player)>) {
    for (mut transform, mut player) in query.iter_mut() {
        let delta_move = player.direction * player.speed * time.delta_secs();
        player.position += delta_move;

        transform.translation.x = player.position.x;
        transform.translation.y = player.position.y;
    }
}
