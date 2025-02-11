use crate::alien;
use crate::plugins::resolution;
use bevy::prelude::*;

pub struct AlienMovementPlugin;

#[derive(Resource)]
struct AlienMovement {
    direction: f32,
    speed: f32,
}

impl Plugin for AlienMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_alien_movement);
        app.add_systems(Update, alien_movement);
    }
}

fn setup_alien_movement(mut commands: Commands) {
    commands.insert_resource(AlienMovement {
        direction: 1.,
        speed: 50.,
    });
}

fn alien_movement(
    mut query: Query<&mut Transform, With<alien::Alien>>,
    mut alien_movement: ResMut<AlienMovement>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
) {
    for mut transform in query.iter_mut() {
        let mut should_reverse = false;
        transform.translation.x +=
            alien_movement.direction * alien_movement.speed * time.delta_secs();

        if transform.translation.x >= resolution.screen_dimensions.x / 2.0
            || transform.translation.x <= -resolution.screen_dimensions.x / 2.0
        {
            should_reverse = true;
        }
        if should_reverse {
            alien_movement.direction *= -1.0;
        }
    }
}
