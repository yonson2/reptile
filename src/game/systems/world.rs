use bevy::prelude::*;

pub fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<MyPausedState>>,
    mut next_paused_state: ResMut<NextState<MyPausedState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        match state.get() {
            MyPausedState::Paused => next_paused_state.set(MyPausedState::Running),
            MyPausedState::Running => next_paused_state.set(MyPausedState::Paused),
        }
    } else if keys.just_pressed(KeyCode::KeyQ) {
        next_app_state.set(AppState::Menu);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyPausedState {
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldSet;
