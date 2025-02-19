use crate::game;
use bevy::prelude::*;

pub struct TextsPlugin;

impl Plugin for TextsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(Update, (game_over, pause, level_completed, points, enemies));
    }
}

#[derive(Component)]
struct PointsText;

#[derive(Component)]
struct EnemiesText;

#[derive(Component)]
pub struct LevelCompleted;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PausedText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/font.ttf");
    let text_font_default = TextFont {
        font: font.clone(),
        font_size: 16.,
        ..default()
    };

    let text_font_big = TextFont {
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
                text_font_default.clone(),
            ));
            parent.spawn((
                TextSpan::default(),
                Text::new("0"),
                text_font_default.clone(),
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
                text_font_default.clone(),
            ));
            parent.spawn((
                TextSpan::default(),
                Text::new("0/0"),
                text_font_default.clone(),
                EnemiesText,
            ));
        });

    commands
        .spawn((Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("GAME OVER"),
                text_font_big.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
                GameOverText,
                Visibility::Hidden,
            ));
        });

    commands
        .spawn((Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                LevelCompleted,
                Text2d::new("LEVEL COMPLETED!"),
                text_font_big.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
                TextSpan::default(),
                Visibility::Hidden,
            ));
            parent.spawn((
                LevelCompleted,
                Text2d::new("Press any key to continue to the next level"),
                text_font_default.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    margin: UiRect::bottom(Val::Px(50.)),
                    ..default()
                },
                TextSpan::default(),
                Visibility::Hidden,
            ));
        });

    commands
        .spawn((Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                PausedText,
                Text2d::new("PAUSED"),
                text_font_big.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
                TextSpan::default(),
                Visibility::Hidden,
            ));
            parent.spawn((
                PausedText,
                Text2d::new("Press P to resume"),
                text_font_default.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    margin: UiRect::bottom(Val::Px(50.)),
                    ..default()
                },
                TextSpan::default(),
                Visibility::Hidden,
            ));
        });
}

fn level_completed(
    state: Res<State<game::GameState>>,
    mut query: Query<&mut Visibility, With<LevelCompleted>>,
) {
    if *state.get() == game::GameState::LevelCompleted {
        for mut text in query.iter_mut() {
            text.toggle_visible_hidden();
        }
    }
}

fn game_over(
    state: Res<State<game::GameState>>,
    mut query: Query<&mut Visibility, With<GameOverText>>,
) {
    if *state.get() == game::GameState::GameOver {
        let mut text = query.single_mut();
        text.toggle_visible_hidden();
    }
}

fn pause(state: Res<State<game::GameState>>, mut query: Query<&mut Visibility, With<PausedText>>) {
    if *state.get() == game::GameState::Paused {
        for mut text in query.iter_mut() {
            text.toggle_visible_hidden();
        }
    }
}

fn points(mut query: Query<&mut Text, With<PointsText>>, game: Res<game::Game>) {
    let mut text = query.single_mut();
    text.0 = format!("{}", game.get_points());
}

fn enemies(mut query: Query<&mut Text, With<EnemiesText>>, game: Res<game::Game>) {
    let mut text = query.single_mut();
    text.0 = format!("{}/{}", game.get_alive_enemies(), game.get_total_enemies());
}
