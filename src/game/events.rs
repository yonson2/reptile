use bevy::prelude::Event;

#[derive(Event)]
pub(super) struct GrowthEvent;

#[derive(Event)]
pub(super) struct GameOverEvent;

#[derive(Event)]
pub(super) struct FoodEvent;
