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
    for (projectile_entity, projectile_transform, _projectile, projectile_collider) in
        query_projectiles.iter()
    {
        for (entity_alien, alien_transform, _alien, alien_collider) in query_enemies.iter() {
            let bullet_position = projectile_transform.translation.truncate();
            let alien_position = alien_transform.translation.truncate();

            let collision_x = (bullet_position.x - alien_position.x).abs()
                < (projectile_collider.width + alien_collider.width) * 0.5;
            let collision_y = (bullet_position.y - alien_position.y).abs()
                < (projectile_collider.height + alien_collider.height) * 0.5;

            if collision_x && collision_y {
                commands.entity(projectile_entity).despawn();
                commands.entity(entity_alien).despawn();
                store.update_points();
            }
        }
    }
}
