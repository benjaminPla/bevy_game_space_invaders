use crate::animations;
use crate::plugins::alien_death;
use crate::plugins::resolution;
use bevy::prelude::*;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_alien);
    }
}

#[derive(Component)]
pub struct Alien;

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 35.;

fn setup_alien(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    resolution: Res<resolution::Resolution>,
) {
    let texture = asset_server.load("alien.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * SPACING * 1.5)
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);

            let animation_config = animations::AnimationConfig::new(0, 1, 5);

            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: animation_config.get_first_sprite_index(),
                    }),
                    ..default()
                },
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                alien_death::Collider::new(),
                Alien,
                animation_config,
            ));
        }
    }
}
