use crate::player;
use crate::plugins::alien_death;
use crate::plugins::resolution;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerShootingPlugin;

impl Plugin for PlayerShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_shooting);
    }
}

#[derive(Component)]
pub struct Bullet;

fn player_shooting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut player::Player)>,
    resolution: Res<resolution::Resolution>,
) {
    for (mut _transform, mut player) in query.iter_mut() {
        player.update_shoot_timer(time.delta_secs());

        let shooting_press = keys.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left);

        if shooting_press && player.get_can_shoot() {
            let texture = asset_server.load("shoot.png");

            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    ..default()
                },
                Transform::from_xyz(
                    player.get_position().x,
                    (resolution.screen_dimensions.y / 3.5) * -1.,
                    0.,
                )
                .with_scale(Vec3::splat(resolution.pixel_ratio)),
                alien_death::Collider::new(5., 5.),
                Bullet,
            ));
            player.reset_shoot_time();
        }
    }
}
