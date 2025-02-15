use crate::animations;
use crate::collisions;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct Enemy;

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 35.;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();

    let texture = asset_server.load("alien.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * SPACING * 1.5)
                + (Vec3::Y * window.height() * 0.5);

            let animation_config = animations::AnimationConfig::new(0, 1, 4);

            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: animation_config.get_first_sprite_index(),
                    }),
                    ..default()
                },
                Transform::from_translation(position),
                collisions::Collider::default(),
                Enemy,
                animation_config,
            ));
        }
    }
}
