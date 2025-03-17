use bevy::prelude::*;

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
