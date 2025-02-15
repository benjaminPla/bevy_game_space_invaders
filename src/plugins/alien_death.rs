use crate::alien;
use crate::plugins::player_shooting;
use crate::plugins::store;
use bevy::prelude::*;

pub struct AlienDeathPlugin;

impl Plugin for AlienDeathPlugin {
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
    query_bullet: Query<(Entity, &Transform, &player_shooting::Bullet, &Collider)>,
    query_alien: Query<(Entity, &Transform, &alien::Alien, &Collider)>,
    mut store: ResMut<store::Store>,
) {
    for (entity_bullet, bullet_transform, _bullet, bullet_collider) in query_bullet.iter() {
        for (entity_alien, alien_transform, _alien, _alien_collider) in query_alien.iter() {
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
