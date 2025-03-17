use bevy::prelude::*;

pub fn global_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<PausedState>>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        match state.get() {
            PausedState::Paused => next_paused_state.set(PausedState::Running),
            PausedState::Running => next_paused_state.set(PausedState::Paused),
        }
    } else if keys.just_pressed(KeyCode::KeyQ) {
        next_app_state.set(AppState::Menu);
        if let GameState::GameOver = game_state.get() {
            next_game_state.set(GameState::Playing);
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
    Game,
    #[default]
    Menu,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldSet;
