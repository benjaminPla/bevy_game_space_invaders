use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(Update, (level_cmpleted,));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    GameOver,
    LevelCompleted,
    Paused,
    #[default]
    Playing,
}

#[derive(Resource)]
pub struct Game {
    alive_enemies: u8,
    points: u8,
    total_enemies: u8,
}

impl Game {
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

    pub fn get_points(&self) -> u8 {
        self.points
    }

    pub fn get_alive_enemies(&self) -> u8 {
        self.alive_enemies
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Game::new());
}

fn level_cmpleted(game: Res<Game>, mut next_state: ResMut<NextState<GameState>>) {
    if game.alive_enemies == 0 {
        next_state.set(GameState::LevelCompleted);
    }
}
