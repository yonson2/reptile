use bevy::prelude::Event;
use super::Direction;

#[derive(Event)]
pub(super) struct GrowthEvent;

#[derive(Event)]
pub(super) struct FoodEvent;

#[derive(Event, Debug)]
pub(super) struct ControllerEvent {
    pub direction: Direction,
}
