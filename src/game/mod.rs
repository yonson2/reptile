mod components;
mod constants;
mod events;
mod resources;
mod systems;
mod ui;

use crate::game::{components::*, events::*, resources::*, systems::world::*, systems::*};
use bevy::{prelude::*, time::common_conditions::on_timer};
use ui::{cleanup_scoreboard, setup_scoreboard};

use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .init_state::<MyPausedState>()
        .init_state::<AppState>()
        .configure_sets(
            Update,
            WorldSet
                .run_if(in_state(MyPausedState::Running))
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(Update, toggle_pause)
        .insert_resource(Score::default())
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(Direction::Up)
        .add_systems(OnEnter(AppState::Menu), ui::menu::setup_menu)
        .add_systems(OnExit(AppState::Menu), ui::menu::cleanup_menu)
        .add_systems(Update, ui::menu::menu.run_if(in_state(AppState::Menu)))
        // .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(OnEnter(AppState::Game), (setup_game, setup_scoreboard))
        .add_systems(OnExit(AppState::Game), (cleanup_game, cleanup_scoreboard))
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
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_event::<FoodEvent>()
        .add_event::<GrowthEvent>();
}
