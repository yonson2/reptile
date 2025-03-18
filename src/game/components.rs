use bevy::prelude::*;

#[derive(Component, PartialEq, Copy, Clone, Default, Debug)]
pub(super) enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
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

    pub fn to_keycode(self) -> KeyCode {
        match self {
            Self::Left => KeyCode::ArrowLeft,
            Self::Right => KeyCode::ArrowRight,
            Self::Up => KeyCode::ArrowUp,
            Self::Down => KeyCode::ArrowDown,
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

#[derive(Component)]
pub(super) struct UserInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArbitraryPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component, Clone, Copy, PartialEq)]
pub enum Position {
    Arbitrary(ArbitraryPosition),
    Fixed(FixedPosition),
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Height(pub f32);

impl Default for Height {
    fn default() -> Self {
        Self(0.)
    }
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
pub struct Controller;
