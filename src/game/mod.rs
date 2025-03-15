mod components;
mod constants;
mod events;
mod resources;
mod systems;

use crate::game::{components::*, events::*, resources::*, systems::*};
use bevy::{prelude::*, time::common_conditions::on_timer};

use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(Direction::Up)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement_input.before(snake_movement))
        .add_systems(
            Update,
            snake_movement.run_if(on_timer(Duration::from_secs_f32(0.15))),
        )
        .add_systems(Update, snake_eating.after(snake_movement))
        .add_systems(Update, snake_growth.after(snake_eating))
        .add_systems(Update, snake_repaint.after(snake_growth))
        .add_systems(Update, game_over.after(snake_movement))
        .add_systems(Update, spawn_food_empty_position)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_event::<FoodEvent>()
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>();
}
