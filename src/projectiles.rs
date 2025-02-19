use crate::constants;
use crate::game;
use bevy::prelude::*;

#[derive(Component)]
pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.run_if(in_state(game::GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Projectile;

fn movement(mut query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, _projectile) in query.iter_mut() {
        transform.translation.y += constants::PROJECTILE_SPEED * time.delta_secs();
    }
}
