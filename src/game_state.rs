use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(Update, (update_points_text, update_enemies_text));
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

    pub fn update_alive_enemies(&mut self) {
        self.alive_enemies -= 1;
    }
}

#[derive(Component)]
struct GameStatePointsText;

#[derive(Component)]
struct GameStateEnemiesText;

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
            top: Val::Px(8.),
            left: Val::Px(8.),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::default(),
                Text::new("Points: "),
                text_font.clone(),
            ));
            parent.spawn((
                TextSpan::default(),
                Text::new("0"),
                text_font.clone(),
                GameStatePointsText,
            ));
        });

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Px(24.),
            left: Val::Px(9.),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::default(),
                Text::new("Enemies: "),
                text_font.clone(),
            ));
            parent.spawn((
                TextSpan::default(),
                Text::new("0/0"),
                text_font.clone(),
                GameStateEnemiesText,
            ));
        });
}

fn update_points_text(
    mut query: Query<&mut Text, With<GameStatePointsText>>,
    store: Res<GameState>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("{}", store.points);
    }
}

fn update_enemies_text(
    mut query: Query<&mut Text, With<GameStateEnemiesText>>,
    store: Res<GameState>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("{}/{}", store.alive_enemies, store.total_enemies);
    }
}
