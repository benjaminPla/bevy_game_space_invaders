use crate::enemy;
use crate::game;
use bevy::prelude::*;

pub struct EnemyMovementPlugin;

#[derive(Resource)]
pub struct EnemyMovement {
    direction: f32,
    speed: f32,
}

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(Update, movement.run_if(in_state(game::GameState::Playing)));
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(EnemyMovement {
        direction: 1.,
        speed: 200.,
    });
}

fn movement(
    mut alien_movement: ResMut<EnemyMovement>,
    mut query: Query<(&mut Transform, &enemy::Enemy)>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();

    let mut should_flip_direction = false;
    let mut should_go_down = false;

    for (transform, _alien) in query.iter() {
        if transform.translation.x.abs() >= window.width() / 2.7 {
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
