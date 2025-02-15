use bevy::prelude::*;

pub struct StorePlugin;

impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_store);
        app.add_systems(Update, update_points_text);
    }
}

#[derive(Resource)]
pub struct Store {
    points: u8,
    enemies: (u8, u8),
}

impl Store {
    fn new() -> Self {
        Self {
            points: 0,
            enemies: (0, 0),
        }
    }

    pub fn update_points(&mut self) {
        self.points += 1;
    }
}

#[derive(Component)]
struct GamePointsText;

fn setup_store(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Store::new());

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
                GamePointsText,
            ));
        });
}

fn update_points_text(mut query: Query<&mut Text, With<GamePointsText>>, store: Res<Store>) {
    for mut points_text in query.iter_mut() {
        points_text.0 = format!("Points {}", store.points);
    }
}
