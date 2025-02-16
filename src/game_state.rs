use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(Update, update_points_text);
    }
}

#[derive(Resource)]
pub struct GameState {
    alive_enemies: u8,
    points: u8,
    total_enemies: u8,
}

impl GameState {
    fn new() -> Self {
        let total_enemies = Self::get_enemies_for_level(1);

        Self {
            alive_enemies: total_enemies,
            points: 0,
            total_enemies,
        }
    }

    pub fn update_points(&mut self) {
        self.points += 1;
    }

    fn get_enemies_for_level(level: u8) -> u8 {
        match level {
            1 => 10,
            2 => 15,
            3 => 20,
            4 => 25,
            _ => 30,
        }
    }

    pub fn get_total_enemies(&self) -> u8 {
        self.total_enemies
    }
}

#[derive(Component)]
struct GameStatePointsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameState::new());

    let font = asset_server.load("fonts/font.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 16.,
        ..default()
    };

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Px(32.0),
            left: Val::Px(32.0),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::default(),
                Text::new("Points 0"),
                text_font.clone(),
                GameStatePointsText,
            ));
        });
}

fn update_points_text(
    mut query: Query<&mut Text, With<GameStatePointsText>>,
    store: Res<GameState>,
) {
    for mut points_text in query.iter_mut() {
        points_text.0 = format!("Points {}", store.points);
    }
}
