use bevy::prelude::*;

pub fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<MyPausedState>>,
    mut next_state: ResMut<NextState<MyPausedState>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        match state.get() {
            MyPausedState::Paused => next_state.set(MyPausedState::Running),
            MyPausedState::Running => next_state.set(MyPausedState::Paused),
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyPausedState {
    #[default]
    Running,
    Paused,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldSet;
