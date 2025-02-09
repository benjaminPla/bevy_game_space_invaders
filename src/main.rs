use bevy::prelude::*;

#[derive(Component)]
struct Player {
    position: Vec2,
    speed: f32,
    direction: Vec2,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player {
            position: Vec2::ZERO,
            speed: 200.0,
            direction: Vec2::ZERO,
        },
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(100., 0., 0.),
    ));
}

fn player_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Player>) {
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

fn player_movement(time: Res<Time>, mut query: Query<(&mut Transform, &mut Player)>) {
    for (mut transform, mut player) in query.iter_mut() {
        let delta_move = player.direction * player.speed * time.delta_secs();
        player.position += delta_move;

        transform.translation.x = player.position.x;
        transform.translation.y = player.position.y;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, player_movement))
        .run();
}
