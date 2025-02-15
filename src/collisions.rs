use crate::enemy;
use crate::game_state;
use crate::projectiles;
use bevy::prelude::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection_system);
    }
}

#[derive(Component)]
pub struct Collider {
    width: f32,
    height: f32,
}

impl Collider {
    pub fn default() -> Self {
        Self {
            width: 32.0,
            height: 32.0,
        }
    }

    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

fn collision_detection_system(
    mut commands: Commands,
    query_projectiles: Query<(Entity, &Transform, &projectiles::Projectile, &Collider)>,
    query_enemies: Query<(Entity, &Transform, &enemy::Enemy, &Collider)>,
    mut store: ResMut<game_state::GameState>,
) {
    for (entity_bullet, bullet_transform, _bullet, bullet_collider) in query_projectiles.iter() {
        for (entity_alien, alien_transform, _alien, _alien_collider) in query_enemies.iter() {
            let bullet_position = bullet_transform.translation.truncate();
            let alien_position = alien_transform.translation.truncate();

            let distance_x = (bullet_position.x - alien_position.x).abs();
            let distance_y = (bullet_position.y - alien_position.y).abs();

            if distance_x < bullet_collider.width && distance_y < bullet_collider.height {
                commands.entity(entity_bullet).despawn();
                commands.entity(entity_alien).despawn();
                store.update_points();
            }
        }
    }
}
