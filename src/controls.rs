use crate::collisions;
use crate::game;
use crate::player;
use crate::projectiles;
use crate::sound;
use crate::texts;
use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, shoot).run_if(in_state(game::GameState::Playing)),
        )
        .add_systems(
            Update,
            pause.run_if(in_state(game::GameState::Playing).or(in_state(game::GameState::Paused))),
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
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<game::GameState>>,
    mut text_events: EventWriter<texts::TextEvents>,
    state: Res<State<game::GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        match state.get() {
            game::GameState::Playing => {
                next_state.set(game::GameState::Paused);
                text_events.send(texts::TextEvents::Paused);
            }
            game::GameState::Paused => {
                next_state.set(game::GameState::Playing);
                text_events.send(texts::TextEvents::Clear);
            }
            _ => {}
        }
    }
}

fn shoot(
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut query_player: Query<&mut player::Player>,
    mut sound_events: EventWriter<sound::SoundEvents>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let mut player = query_player.single_mut();
    let window = window_query.single();

    player.update_shoot_timer(time.delta_secs());

    let shooting_press = keys.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left);

    if shooting_press && player.get_can_shoot() {
        let texture = asset_server.load("shoot.png");

        commands.spawn((
            Sprite {
                image: texture.clone(),
                ..default()
            },
            Transform::from_xyz(player.get_position().x, window.height() / 3.5 * -1., 0.),
            collisions::Collider::new(5., 32.),
            projectiles::Projectile,
        ));
        player.reset_shoot_time();
        sound_events.send(sound::SoundEvents::Projectile);
    }
}
