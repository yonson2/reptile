use crate::{
    assets::{ControllerAsset, ImageAsset},
    game::{
        constants::*, AppState, ArbitraryPosition, Controller, Direction, MainGameScreen, Position,
        Size,
    },
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, insert_controller)
        .add_systems(Update, controller_input);
}

fn insert_controller(mut commands: Commands, controller_asset: Res<ControllerAsset>) {
    let controller_buttons = [
        (CONTROLLER_UP, 3.5, 1.75, Direction::Up),
        (CONTROLLER_DOWN, 3.5, 0.25, Direction::Down),
        (CONTROLLER_LEFT, 2.75, 1.0, Direction::Left),
        (CONTROLLER_RIGHT, 4.25, 1.0, Direction::Right),
    ];

    for (index, x, y, dir) in controller_buttons {
        spawn_controller_button(&mut commands, &controller_asset, index, x, y, dir);
    }
}

/// System to handle controller button clicks
fn controller_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    controller_sprites: Query<(&GlobalTransform, &Direction, &Position), With<Controller>>,
) {
    // Only process clicks, not holds
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the window and cursor position
    let window = window_q.get_single().unwrap();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Get the camera to convert screen coordinates to world coordinates
    let (camera, camera_transform) = camera_q.get_single().unwrap();
    let Ok(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };

    // Get the window dimensions for scaling calculations
    let window_width = window.width();
    let window_height = window.height();

    // Calculate tile size
    let tile_width = window_width / ARENA_WIDTH as f32;
    let tile_height = window_height / ARENA_HEIGHT as f32;

    // Check if any controller sprite was clicked
    for (transform, direction, _) in controller_sprites.iter() {
        let sprite_position = transform.translation().truncate();

        // Calculate the expected size of the controller button based on the window size
        let button_size_x = tile_width;
        let button_size_y = tile_height;

        // Use a larger hit area for the controller buttons
        let half_size_x = button_size_x * 0.35;
        let half_size_y = button_size_y * 0.35;

        if cursor_world_position.x >= sprite_position.x - half_size_x
            && cursor_world_position.x <= sprite_position.x + half_size_x
            && cursor_world_position.y >= sprite_position.y - half_size_y
            && cursor_world_position.y <= sprite_position.y + half_size_y
        {
            // Controller button was clicked, update the input direction
            info!("Controller button clicked: {:?}", direction);
        }
    }
}

/// Helper function to spawn a controller button with the given parameters
fn spawn_controller_button(
    commands: &mut Commands,
    controller_asset: &ControllerAsset,
    index: usize,
    x: f32,
    y: f32,
    dir: Direction,
) {
    commands
        .spawn(Sprite::from_atlas_image(
            controller_asset.0.texture.clone(),
            TextureAtlas {
                layout: controller_asset.0.atlas_layout.clone(),
                index,
            },
        ))
        .insert(dir)
        .insert(Controller)
        .insert(ImageAsset)
        .insert(MainGameScreen)
        .insert(Position::Arbitrary(ArbitraryPosition { x, y }))
        .insert(Size::square(1.));
}

