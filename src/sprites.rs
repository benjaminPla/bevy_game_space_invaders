use bevy::prelude::*;

#[derive(Component)]
pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource)]
pub struct SpritesResource {
    pub enemy: Handle<Image>,
    pub planet: Handle<Image>,
    pub player: Handle<Image>,
    pub projectile: Handle<Image>,
    pub star_big: Handle<Image>,
    pub star_small: Handle<Image>,
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let sprites_resource = SpritesResource {
        enemy: asset_server.load("sprites/enemy.png"),
        planet: asset_server.load("sprites/planet.png"),
        player: asset_server.load("sprites/player.png"),
        projectile: asset_server.load("sprites/projectile.png"),
        star_big: asset_server.load("sprites/star_big.png"),
        star_small: asset_server.load("sprites/star_small.png"),
    };
    commands.insert_resource(sprites_resource);
}
