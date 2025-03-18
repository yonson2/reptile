use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::components::*;
use crate::game::constants::*;

pub fn global_input(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    state: Res<State<PausedState>>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keys.clear_just_pressed(KeyCode::KeyP) {
        match state.get() {
            PausedState::Paused => next_paused_state.set(PausedState::Running),
            PausedState::Running => next_paused_state.set(PausedState::Paused),
        }
    } else if keys.clear_just_pressed(KeyCode::KeyQ) {
        next_app_state.set(AppState::Menu);
    }
}

pub fn size_scaling(
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    mut q_scale: Query<(&Size, &mut Transform, Option<&ImageAsset>)>,
) {
    if let Some(window) = window {
        let tile_width = window.width() / ARENA_WIDTH as f32;
        let tile_height = window.height() / ARENA_HEIGHT as f32;

        for (sprite_size, mut transform, is_image) in &mut q_scale {
            if is_image.is_some() {
                let sprite_pixel_size = SPRITE_PIXEL_SIZE;

                transform.scale = Vec3::new(
                    tile_width * sprite_size.width / sprite_pixel_size,
                    tile_height * sprite_size.height / sprite_pixel_size,
                    1.0,
                );
            } else {
                transform.scale = Vec3::new(
                    sprite_size.width / ARENA_WIDTH as f32 * window.width(),
                    sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
                    1.0,
                );
            }
        }
    }
}

pub fn position_translation(
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    if let Some(window) = window {
        fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
            let tile_size = bound_window / bound_game;
            pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
        }
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
                0.0,
            );
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldSet;
