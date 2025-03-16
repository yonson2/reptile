use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAsset {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct AudioAsset(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct TextAsset(pub Handle<Font>);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreStartup, load_assets);
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the snake texture
    let texture = asset_server.load("sprites/snake.png");
    let audio = asset_server.load("sounds/crunchybite.ogg");
    let font = asset_server.load("fonts/fibberish.ttf");

    // Create a texture atlas layout for the snake
    // The index calculation is the key part:
    // to get a specific sprite at column C, row R in a texture atlas with W columns,
    // the formula is `R * W + C` (for 0-based indexing)
    // or `(R-1) * W + (C-1)` (for 1-based indexing).
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 16, 22, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);

    // Store the assets in a resource so they can be accessed from other systems
    commands.insert_resource(GameAsset {
        texture,
        atlas_layout,
    });

    commands.insert_resource(AudioAsset(audio));
    commands.insert_resource(TextAsset(font));

    // Log that assets were loaded successfully
    info!("Game assets loaded successfully");
}
