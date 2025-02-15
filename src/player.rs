use crate::animations;
use crate::plugins::resolution;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

const SHOOT_DELAY: f32 = 0.5;

#[derive(Component)]
pub struct Player {
    can_shoot: bool,
    position: Vec2,
    pub shoot_timer: Timer,
    speed: f32,
}

impl Player {
    pub fn new(resolution: &Res<resolution::Resolution>) -> Self {
        Self {
            can_shoot: true,
            position: Vec2::new(0., (resolution.screen_dimensions.y / 3.5) * -1.),
            shoot_timer: Timer::new(Duration::from_secs_f32(SHOOT_DELAY), TimerMode::Once),
            speed: 200.0,
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position_x(&mut self, new_position_x: f32) {
        self.position.x = new_position_x;
    }

    pub fn get_can_shoot(&self) -> bool {
        self.can_shoot
    }

    pub fn reset_shoot_time(&mut self) {
        if self.can_shoot {
            self.can_shoot = false;
            self.shoot_timer.reset();
        }
    }

    pub fn update_shoot_timer(&mut self, delta_time: f32) {
        if !self.can_shoot {
            self.shoot_timer.tick(Duration::from_secs_f32(delta_time));

            if self.shoot_timer.finished() {
                self.can_shoot = true;
            }
        }
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

    let animation_config = animations::AnimationConfig::new(0, 1, 2);

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
        Player::new(&resolution),
        animation_config,
    ));
}
