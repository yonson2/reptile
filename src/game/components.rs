use bevy::prelude::*;

#[derive(Component, Resource, PartialEq, Copy, Clone)]
pub(super) enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
pub(super) struct SnakeHead;

#[derive(Component)]
pub(super) struct SnakeBody;

#[derive(Component)]
pub(super) struct Food;

#[derive(Component)]
pub(super) struct ScoreboardUi;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub(super) struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub(super) struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
pub(super) struct ImageAsset;
