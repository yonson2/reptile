use bevy::prelude::*;

use super::Position;

#[derive(Default, Resource)]
pub(super) struct SnakeSegments(pub Vec<Entity>);

#[derive(Default, Resource)]
pub(super) struct LastTailPosition(pub Option<Position>);

#[derive(Default, Resource)]
pub(super) struct Score(pub usize);
