use crate::enemy;
use crate::game;
use crate::player;
use crate::projectiles;
use bevy::prelude::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_enemy, enemy_boundary, projectile_enemy)
                .run_if(in_state(game::GameState::Playing)),
        )
        .add_systems(Update, projectile_boundary);
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

fn projectile_enemy(
    mut commands: Commands,
    query_projectiles: Query<(Entity, &Transform, &Collider), With<projectiles::Projectile>>,
    query_enemies: Query<(Entity, &Transform, &Collider), With<enemy::Enemy>>,
    mut game: ResMut<game::Game>,
) {
    for (projectile_entity, projectile_transform, projectile_collider) in query_projectiles.iter() {
        for (enemy_entity, enemy_transform, enemy_collider) in query_enemies.iter() {
            let bullet_position = projectile_transform.translation.truncate();
            let enemy_position = enemy_transform.translation.truncate();

            let collision_x = (bullet_position.x - enemy_position.x).abs()
                < (projectile_collider.width + enemy_collider.width) * 0.5;
            let collision_y = (bullet_position.y - enemy_position.y).abs()
                < (projectile_collider.height + enemy_collider.height) * 0.5;

            if collision_x && collision_y {
                commands.entity(projectile_entity).despawn();
                commands.entity(enemy_entity).despawn();
                game.update_points();
                game.update_alive_enemies();
            }
        }
    }
}

fn projectile_boundary(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<projectiles::Projectile>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    for (entity, transform) in query.iter() {
        if transform.translation.y > window.height() * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

fn enemy_boundary(
    query: Query<&Transform, With<enemy::Enemy>>,
    window_query: Query<&Window>,
    mut next_state: ResMut<NextState<game::GameState>>,
) {
    let window = window_query.single();
    for transform in query.iter() {
        if transform.translation.y <= -window.height() * 0.5 {
            next_state.set(game::GameState::GameOver);
        }
    }
}

fn player_enemy(
    query_player: Query<(&Transform, &Collider), With<player::Player>>,
    query_enemies: Query<(&Transform, &Collider), With<enemy::Enemy>>,
    mut next_state: ResMut<NextState<game::GameState>>,
) {
    let (player_transform, player_collider) = query_player.single();
    for (enemy_transform, enemy_collider) in query_enemies.iter() {
        let player_position = player_transform.translation.truncate();
        let enemy_position = enemy_transform.translation.truncate();

        let collision_x = (player_position.x - enemy_position.x).abs()
            < (player_collider.width + enemy_collider.width) * 0.5;
        let collision_y = (player_position.y - enemy_position.y).abs()
            < (player_collider.height + enemy_collider.height) * 0.5;

        if collision_x && collision_y {
            next_state.set(game::GameState::GameOver);
        }
    }
}
