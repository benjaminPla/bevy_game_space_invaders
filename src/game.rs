use crate::assets;
use crate::enemy;
use crate::sound;
use crate::texts;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
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
    #[default]
    Loading,
    Paused,
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
            Level::One => 15,
            Level::Two => 20,
            Level::Three => 25,
            Level::Four => 30,
            Level::Five => 35,
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

fn level_completed(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sound_events: EventWriter<sound::SoundEvents>,
    mut text_events: EventWriter<texts::TextEvents>,
) {
    if game.alive_enemies == 0 {
        match game.get_next_level() {
            Some(next_level) => {
                text_events.send(texts::TextEvents::LevelCompleted);
                next_state.set(GameState::LevelCompleted);
                let next_enemies = Game::get_enemies_for_level(&next_level);
                game.set_next_level(next_level);
                game.set_alive_enemies(next_enemies);
                game.set_total_enemies(next_enemies);
            }
            None => {
                sound_events.send(sound::SoundEvents::LevelCompleted);
                text_events.send(texts::TextEvents::GameCompleted);
                next_state.set(GameState::GameCompleted);
            }
        };
    }
}

fn jump_to_next_lvl(
    commands: Commands,
    game: Res<Game>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut text_events: EventWriter<texts::TextEvents>,
    sprites_resource: Res<assets::SpriteAssets>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        enemy::setup(
            game,
            commands,
            texture_atlas_layouts,
            sprites_resource,
            window_query,
        );
        next_state.set(GameState::Playing);
        text_events.send(texts::TextEvents::Clear);
    }
}
