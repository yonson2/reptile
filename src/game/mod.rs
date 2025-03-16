mod components;
mod constants;
mod events;
mod menu;
mod resources;
mod systems;

use crate::game::{components::*, events::*, resources::*, systems::world::*, systems::*};
use bevy::{prelude::*, time::common_conditions::on_timer};

use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .init_state::<MyPausedState>()
        .init_state::<AppState>()
        .configure_sets(
            Update,
            WorldSet
                .run_if(in_state(MyPausedState::Running))
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, toggle_pause)
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(Direction::Up)
        .add_systems(OnEnter(AppState::Menu), menu::setup_menu)
        .add_systems(OnExit(AppState::Menu), menu::cleanup_menu)
        .add_systems(Update, menu::menu.run_if(in_state(AppState::Menu)))
        // .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(OnExit(AppState::InGame), cleanup_game)
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
