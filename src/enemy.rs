use crate::animations;
use crate::collisions;
use crate::constants;
use crate::game;
use crate::sprites;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
    }
}

#[derive(Component)]
pub struct Enemy;

pub fn setup(
    game: Res<game::Game>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprites_resource: Res<sprites::SpritesResource>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let texture = sprites_resource.enemy.clone();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let total_enemies = game.get_total_enemies() as u32;

    let mut enemies_spawned = 0;
    let mut row = 0;
    let mut enemies_in_row = 1;

    while (enemies_in_row * (enemies_in_row + 1)) / 2 < total_enemies {
        enemies_in_row += 1;
    }

    while enemies_spawned < total_enemies {
        let start_x = -(enemies_in_row as f32 * constants::ENEMIES_GAP * 0.5);
        let row_enemies = enemies_in_row.min(total_enemies - enemies_spawned);

        for i in 0..row_enemies {
            let position = Vec3::new(
                start_x + i as f32 * constants::ENEMIES_GAP,
                window.height() * 0.5 - row as f32 * constants::ENEMIES_GAP - 100.0,
                0.,
            );

            let animation_config = animations::Animations::new(0, 1, 4);

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

            enemies_spawned += 1;
        }

        row += 1;
        enemies_in_row -= 1;
    }
}
