use crate::collisions;
use crate::constants;
use crate::player;
use bevy::prelude::*;

#[derive(Component)]
pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (despawn, movement, spawn));
    }
}

#[derive(Component)]
pub struct Projectile;

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut player_query: Query<&mut player::Player>,
    window_query: Query<&Window>,
) {
    let mut player = player_query.single_mut();
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

fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    for (entity, transform) in query.iter() {
        if transform.translation.y > window.height() * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

fn movement(mut query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, _projectile) in query.iter_mut() {
        transform.translation.y += constants::PROJECTILE_SPEED * time.delta_secs();
    }
}
