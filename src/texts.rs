use crate::game;
use bevy::prelude::*;

pub struct TextsPlugin;

impl Plugin for TextsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_event::<TextEvents>()
            .add_systems(Update, (points, enemies, handle_text_events));
    }
}

#[derive(Event)]
pub enum TextEvents {
    GameCompleted,
    GameOver,
    Clear,
    LevelCompleted,
    Paused,
}

#[derive(Component)]
struct PointsText;

#[derive(Component)]
struct EnemiesText;

#[derive(Component)]
struct PrimaryText;

#[derive(Component)]
struct SecondaryText;

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
                Text2d::new(""),
                text_font_big.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
                PrimaryText,
            ));
            parent.spawn((
                Text2d::new(""),
                text_font_default.clone(),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    margin: UiRect::bottom(Val::Px(50.)),
                    ..default()
                },
                SecondaryText,
            ));
        });
}

fn handle_text_events(
    mut events: EventReader<TextEvents>,
    mut primary_text_query: Query<&mut Text2d, With<PrimaryText>>,
    mut secondary_text_query: Query<&mut Text2d, (With<SecondaryText>, Without<PrimaryText>)>,
) {
    for event in events.read() {
        let mut primary_text = primary_text_query.single_mut();
        let mut secondary_text = secondary_text_query.single_mut();
        match event {
            TextEvents::LevelCompleted => {
                primary_text.0 = "LEVEL COMPLETED!".to_string();
                secondary_text.0 = "Press space key to continue to the next level".to_string();
            }
            TextEvents::GameOver => {
                primary_text.0 = "GAME OVER".to_string();
                secondary_text.0 = "".to_string();
            }
            TextEvents::GameCompleted => {
                primary_text.0 = "GAME COMPLETED!".to_string();
                secondary_text.0 = "".to_string();
            }
            TextEvents::Paused => {
                primary_text.0 = "PAUSED".to_string();
                secondary_text.0 = "Press P to resume.".to_string();
            }
            TextEvents::Clear => {
                primary_text.0 = "".to_string();
                secondary_text.0 = "".to_string();
            }
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
