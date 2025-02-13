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
        speed: 200.,
    });
}

fn alien_movement(
    mut query: Query<(&mut Transform, &alien::Alien)>,
    mut alien_movement: ResMut<AlienMovement>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
) {
    let mut should_flip_direction = false;
    let mut should_go_down = false;

    for (transform, _alien) in query.iter() {
        if transform.translation.x.abs() >= resolution.screen_dimensions.x / 2.7 {
            should_flip_direction = true;
            should_go_down = true;
            break;
        }
    }

    if should_flip_direction {
        alien_movement.direction *= -1.;
    }

    for (mut transform, _alien) in query.iter_mut() {
        if should_go_down {
            transform.translation.y -= 35.;
        }

        transform.translation.x +=
            alien_movement.direction * alien_movement.speed * time.delta_secs();
    }
}
