use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(Update, (update_points_text, update_enemies_text));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    GameOver,
    LevelWon,
    Paused,
    #[default]
    Playing,
}

#[derive(Resource)]
pub struct Game {
    alive_enemies: u8,
    points: u8,
    state: GameState,
    total_enemies: u8,
}

impl Game {
    fn new() -> Self {
        let total_enemies = Self::get_enemies_for_level(1);

        Self {
            alive_enemies: total_enemies,
            points: 0,
            state: GameState::Playing,
            total_enemies,
        }
    }

    pub fn update_points(&mut self) {
        self.points += 1;
    }

    fn get_enemies_for_level(level: u8) -> u8 {
        match level {
            1 => 20,
            2 => 25,
            3 => 30,
            4 => 35,
            _ => 40,
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
struct PointsText;

#[derive(Component)]
struct EnemiesText;

#[derive(Component)]
pub struct GameOverText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Game::new());

    let font = asset_server.load("fonts/font.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 16.,
        ..default()
    };

    let text_font_game_over = TextFont {
        font: font.clone(),
        font_size: 64.,
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
                PointsText,
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
                EnemiesText,
            ));
        });

    commands.spawn((
        Text2d::new("GAME OVER"),
        text_font_game_over,
        TextLayout::new_with_justify(JustifyText::Center),
        GameOverText,
        Visibility::Hidden,
    ));
}

fn update_points_text(mut query: Query<&mut Text, With<PointsText>>, game: Res<Game>) {
    let mut text = query.single_mut();
    text.0 = format!("{}", game.points);
}

fn update_enemies_text(mut query: Query<&mut Text, With<EnemiesText>>, game: Res<Game>) {
    let mut text = query.single_mut();
    text.0 = format!("{}/{}", game.alive_enemies, game.total_enemies);
}
