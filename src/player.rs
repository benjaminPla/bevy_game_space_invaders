use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    position: Vec2,
    speed: f32,
    pub direction: Vec2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            speed: 200.0,
            direction: Vec2::ZERO,
        }
    }
}

pub fn player_movement(time: Res<Time>, mut query: Query<(&mut Transform, &mut Player)>) {
    for (mut transform, mut player) in query.iter_mut() {
        let delta_move = player.direction * player.speed * time.delta_secs();
        player.position += delta_move;

        transform.translation.x = player.position.x;
        transform.translation.y = player.position.y;
    }
}
