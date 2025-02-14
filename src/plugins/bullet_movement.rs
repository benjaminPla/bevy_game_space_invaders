use crate::plugins::player_shooting;
use crate::plugins::resolution;
use bevy::prelude::*;

pub struct BulletMovementPlugin;

#[derive(Resource)]
struct BulletMovement {
    speed: f32,
}

impl Plugin for BulletMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bullet_movement_resource);
        app.add_systems(Update, (bullet_despawn_system, bullet_movement_system));
    }
}

fn setup_bullet_movement_resource(mut commands: Commands) {
    commands.insert_resource(BulletMovement { speed: 500. });
}

fn bullet_despawn_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<player_shooting::Bullet>>,
    resolution: Res<resolution::Resolution>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_movement_system(
    mut query: Query<(&mut Transform, &player_shooting::Bullet)>,
    bullet_resource: Res<BulletMovement>,
    time: Res<Time>,
) {
    for (mut transform, _bullet) in query.iter_mut() {
        transform.translation.y += bullet_resource.speed * time.delta_secs();
    }
}
