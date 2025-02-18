use crate::collisions;
use crate::constants;
use crate::game;
use crate::player;
use bevy::prelude::*;

#[derive(Component)]
pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (movement, spawn).run_if(in_state(game::GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Projectile;

fn spawn(
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut query_player: Query<&mut player::Player>,
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
            Projectile,
        ));
        player.reset_shoot_time();
    }
}

fn movement(mut query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, _projectile) in query.iter_mut() {
        transform.translation.y += constants::PROJECTILE_SPEED * time.delta_secs();
    }
}
