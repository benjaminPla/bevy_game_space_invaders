use crate::animations;
use crate::plugins::resolution;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { speed: 200.0 }
   }
}

fn setup_player(
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
        Transform::from_xyz(0., (resolution.screen_dimensions.y / 3.5) * -1., 0.)
            .with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player::new(),
        animation_config,
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    resolution: Res<resolution::Resolution>,
) {
    for (mut transform, player) in query.iter_mut() {
        let boundary_x = resolution.screen_dimensions.x / 2.7;

        let moving_left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
        let moving_right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);

        let mut movement_direction = 0.0;
        if moving_left {
            movement_direction = -1.0;
        }
        if moving_right {
            movement_direction = 1.0;
        }

        let delta_move = movement_direction * player.speed * time.delta_secs();
        let new_position_x = transform.translation.x + delta_move;

        if new_position_x.abs() <= boundary_x {
            transform.translation.x = new_position_x;
        }
    }
}
