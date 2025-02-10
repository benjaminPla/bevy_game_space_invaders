use crate::animations;
use bevy::prelude::*;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_alien);
    }
}

#[derive(Component)]
struct Alien;

pub fn setup_alien(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("alien.png");

    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
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
        Transform::from_xyz(0., 0., 0.),
        Alien,
        animation_config,
    ));
}
