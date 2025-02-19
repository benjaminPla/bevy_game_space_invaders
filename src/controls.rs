use crate::game;
use crate::player;
use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (pause, player_movement).run_if(in_state(game::GameState::Playing)),
        );
    }
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut player::Player)>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();

    for (mut transform, mut player) in query.iter_mut() {
        let boundary_x = window.width() / 2.7;

        let moving_left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
        let moving_right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);

        let mut movement_direction = 0.0;
        if moving_left {
            movement_direction = -1.0;
        }
        if moving_right {
            movement_direction = 1.0;
        }

        let delta_move = movement_direction * player.get_speed() * time.delta_secs();
        let new_position_x = transform.translation.x + delta_move;

        if new_position_x.abs() <= boundary_x {
            transform.translation.x = new_position_x;
            player.set_position_x(new_position_x);
        }
    }
}

fn pause(
    state: Res<State<game::GameState>>,
    mut next_state: ResMut<NextState<game::GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        match state.get() {
            game::GameState::Playing => next_state.set(game::GameState::Paused),
            game::GameState::Paused => next_state.set(game::GameState::Playing),
            _ => {}
        }
    }
}
