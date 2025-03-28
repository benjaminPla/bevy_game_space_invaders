use crate::animations;
use crate::assets;
use crate::constants;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::game::GameState::Playing), setup)
            .add_systems(Update, animate);
    }
}

#[derive(Component)]
struct BackgroundComponent {
    sprite_height: f32,
}

impl BackgroundComponent {
    fn new(sprite_height: f32) -> Self {
        Self { sprite_height }
    }
}

fn setup(
    mut commands: Commands,
    sprites_resource: Res<assets::SpriteAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let mut rng = rand::rng();

    let star_small_texture = sprites_resource.star_small.clone();
    let star_big_texture = sprites_resource.star_big.clone();
    let planet_texture = sprites_resource.planet.clone();

    let star_small_layout = TextureAtlasLayout::from_grid(UVec2::splat(4), 1, 2, None, None);
    let star_big_layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 1, 2, None, None);
    let planet_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 1, 1, None, None);

    let star_small_texture_atlas_layout = texture_atlas_layouts.add(star_small_layout);
    let star_big_texture_atlas_layout = texture_atlas_layouts.add(star_big_layout);
    let planet_texture_atlas_layout = texture_atlas_layouts.add(planet_layout);

    for _ in 0..constants::BACKGROUND_SMALL_STARS {
        let animation_config = animations::Animations::new(0, 1, 2);
        let position = Vec3::new(
            rng.random_range(-window.width() / 2.0..window.width() / 2.0),
            rng.random_range(-window.height() / 2.0..window.height() / 2.0),
            -1.0,
        );
        commands.spawn((
            Sprite {
                image: star_small_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: star_small_texture_atlas_layout.clone(),
                    index: animation_config.get_first_sprite_index(),
                }),
                ..default()
            },
            Transform::from_translation(position),
            BackgroundComponent::new(4.),
            animation_config,
        ));
    }

    for _ in 0..constants::BACKGROUND_BIG_STARS {
        let animation_config = animations::Animations::new(0, 1, 2);
        let position = Vec3::new(
            rng.random_range(-window.width() / 2.0..window.width() / 2.0),
            rng.random_range(-window.height() / 2.0..window.height() / 2.0),
            -1.0,
        );
        commands.spawn((
            Sprite {
                image: star_big_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: star_big_texture_atlas_layout.clone(),
                    index: animation_config.get_first_sprite_index(),
                }),
                ..default()
            },
            Transform::from_translation(position),
            BackgroundComponent::new(8.),
            animation_config,
        ));
    }

    for _ in 0..constants::BACKGROUND_PLANETS {
        let animation_config = animations::Animations::new(0, 1, 2);
        let position = Vec3::new(
            rng.random_range(-window.width() / 2.0..window.width() / 2.0),
            rng.random_range(-window.height() / 2.0..window.height() / 2.0),
            -1.0,
        );
        commands.spawn((
            Sprite {
                image: planet_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: planet_texture_atlas_layout.clone(),
                    index: animation_config.get_first_sprite_index(),
                }),
                ..default()
            },
            Transform::from_translation(position),
            BackgroundComponent::new(32.),
            animation_config,
        ));
    }
}

fn animate(
    mut query: Query<(&mut Transform, &BackgroundComponent)>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let screen_bottom = -window.height() / 2.0;

    for (mut transform, background_component) in query.iter_mut() {
        transform.translation.y -= 50.0 * time.delta_secs();

        let bottom_threshold = screen_bottom - background_component.sprite_height;
        if transform.translation.y < bottom_threshold {
            transform.translation.y = window.height() / 2.0 + background_component.sprite_height;
            transform.translation.x =
                rand::rng().random_range(-window.width() / 2.0..window.width() / 2.0);
        }
    }
}
