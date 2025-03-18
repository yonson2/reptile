use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteAsset {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct SnakeAsset(pub SpriteAsset);

#[derive(Resource)]
pub struct DpadAsset(pub SpriteAsset);

#[derive(Resource)]
pub struct AudioAsset(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct FontAsset(pub Handle<Font>);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreStartup, load_assets);
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/snake.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 16, 22, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(SnakeAsset(SpriteAsset {
        texture,
        atlas_layout,
    }));

    let texture = asset_server.load("sprites/dpad.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 4, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(DpadAsset(SpriteAsset {
        texture,
        atlas_layout,
    }));

    let audio = asset_server.load("sounds/crunchybite.ogg");
    commands.insert_resource(AudioAsset(audio));

    let font = asset_server.load("fonts/fibberish.ttf");
    commands.insert_resource(FontAsset(font));

    // Log that assets were loaded successfully
    info!("Game assets loaded successfully");
}
