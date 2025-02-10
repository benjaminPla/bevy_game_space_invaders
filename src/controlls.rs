use crate::player;
use bevy::prelude::*;

pub fn player_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut player::Player>) {
    let mut movement = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    if let Some(mut player) = query.iter_mut().next() {
        player.direction = if movement.length() > 0.0 {
            movement.normalize()
        } else {
            Vec2::ZERO
        };
    }
}
