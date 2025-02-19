use crate::enemy;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, level_completed)
            .add_systems(
                Update,
                jump_to_next_lvl.run_if(in_state(GameState::LevelCompleted)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    GameCompleted,
    GameOver,
    LevelCompleted,
    Paused,
    #[default]
    Playing,
}

pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Resource)]
pub struct Game {
    alive_enemies: u8,
    level: Level,
    points: u8,
    total_enemies: u8,
}

impl Game {
    fn new() -> Self {
        let level = Level::One;
        let total_enemies = Self::get_enemies_for_level(&level);
        Self {
            alive_enemies: total_enemies,
            level,
            points: 0,
            total_enemies,
        }
    }

    pub fn update_points(&mut self) {
        self.points += 1;
    }

    fn get_enemies_for_level(level: &Level) -> u8 {
        match level {
            // Level::One => 20,
            // Level::Two => 25,
            // Level::Three => 30,
            // Level::Four => 35,
            // Level::Five => 40,
            Level::One => 1,
            Level::Two => 2,
            Level::Three => 3,
            Level::Four => 4,
            Level::Five => 5,
        }
    }

    pub fn update_alive_enemies(&mut self) {
        self.alive_enemies -= 1;
    }

    pub fn get_points(&self) -> u8 {
        self.points
    }

    pub fn get_alive_enemies(&self) -> u8 {
        self.alive_enemies
    }

    pub fn set_alive_enemies(&mut self, enemies: u8) {
        self.alive_enemies = enemies;
    }

    pub fn get_total_enemies(&self) -> u8 {
        self.total_enemies
    }

    pub fn set_total_enemies(&mut self, enemies: u8) {
        self.total_enemies = enemies;
    }

    fn get_next_level(&self) -> Option<Level> {
        match self.level {
            Level::One => Some(Level::Two),
            Level::Two => Some(Level::Three),
            Level::Three => Some(Level::Four),
            Level::Four => Some(Level::Five),
            Level::Five => None,
        }
    }

    fn set_next_level(&mut self, level: Level) {
        self.level = level;
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Game::new());
}

fn level_completed(mut game: ResMut<Game>, mut next_state: ResMut<NextState<GameState>>) {
    if game.alive_enemies == 0 {
        next_state.set(GameState::LevelCompleted);
        match game.get_next_level() {
            Some(next_level) => {
                let next_enemies = Game::get_enemies_for_level(&next_level);
                game.set_next_level(next_level);
                game.set_alive_enemies(next_enemies);
                game.set_total_enemies(next_enemies);
            }
            None => next_state.set(GameState::GameCompleted),
        };
    }
}

fn jump_to_next_lvl(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window>,
    game: Res<Game>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        enemy::setup(
            commands,
            asset_server,
            texture_atlas_layouts,
            window_query,
            game,
        );
        next_state.set(GameState::Playing);
    }
}
