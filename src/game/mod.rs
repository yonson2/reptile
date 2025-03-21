pub mod components;
pub mod constants;
mod events;
mod resources;
pub mod systems;
mod ui;

use crate::{
    despawn_screen,
    game::{
        components::*,
        events::*,
        resources::*,
        systems::{world::*, *},
    },
};
use bevy::{prelude::*, time::common_conditions::on_timer};
#[allow(unused_imports)]
use ui::{
    controller,
    menu::{GameOverScreen, MainMenuScreen},
};

use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    #[allow(dead_code)]
    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugins(controller::plugin);
    }
    app.insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .init_state::<PausedState>()
        .init_state::<AppState>()
        .init_state::<GameState>()
        .configure_sets(
            Update,
            WorldSet
                .run_if(in_state(PausedState::Running))
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            global_input.run_if(not(in_state(AppState::Loading))),
        )
        .insert_resource(Score::default())
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_systems(OnEnter(AppState::Menu), ui::menu::setup_menu)
        .add_systems(OnExit(AppState::Menu), despawn_screen::<MainMenuScreen>)
        .add_systems(OnEnter(AppState::Game), (setup_game, set_playing_state))
        .add_systems(
            OnExit(AppState::Game),
            (
                despawn_screen::<MainGameScreen>,
                despawn_screen::<GameOverScreen>,
            ),
        )
        .add_systems(OnExit(GameState::Playing), despawn_screen::<MainGameScreen>)
        .add_systems(OnEnter(GameState::GameOver), ui::menu::setup_game_over)
        .add_systems(
            OnExit(GameState::GameOver),
            (
                despawn_screen::<GameOverScreen>,
                despawn_screen::<MainGameScreen>,
                setup_game,
            )
                .chain(),
        )
        .add_systems(Update, ui::menu::menu.run_if(in_state(AppState::Menu)))
        // game logic:
        // runs on AppState::Game && GameState::Playing && PausedState::Running.
        .add_systems(
            Update,
            (
                snake_movement_input,
                snake_movement
                    .after(snake_movement_input)
                    .run_if(on_timer(Duration::from_secs_f32(0.15))),
                (snake_eating, snake_growth, snake_repaint)
                    .chain()
                    .after(snake_movement),
                spawn_food_empty_position,
            )
                .in_set(WorldSet),
        )
        .add_systems(
            Update,
            game_over_input.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_event::<FoodEvent>()
        .add_event::<GrowthEvent>();
}

fn set_playing_state(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);
}
