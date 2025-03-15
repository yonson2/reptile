use bevy::color::Color;

// Constants
//opens mouth at +1/2/3/4 (bigger each) for each direciton
pub(super) const SNAKE_HEAD_UP: usize = 48;
pub(super) const SNAKE_HEAD_DOWN: usize = 80;
pub(super) const SNAKE_HEAD_LEFT: usize = 64;
pub(super) const SNAKE_HEAD_RIGHT: usize = 96;

pub(super) const SNAKE_BODY_VERTICAL: usize = 32;
pub(super) const SNAKE_BODY_HORIZONTAL: usize = 33;

pub(super) const SNAKE_CORNER_BOTTOM_RIGHT: usize = 34;
pub(super) const SNAKE_CORNER_BOTTOM_LEFT: usize = 35;
pub(super) const SNAKE_CORNER_TOP_RIGHT: usize = 36;
pub(super) const SNAKE_CORNER_TOP_LEFT: usize = 37;

pub(super) const SNAKE_TAIL_UP: usize = 38;
pub(super) const SNAKE_TAIL_DOWN: usize = 40;
pub(super) const SNAKE_TAIL_LEFT: usize = 39;
pub(super) const SNAKE_TAIL_RIGHT: usize = 41;

pub(super) const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

pub(super) const ARENA_WIDTH: u32 = 11;
pub(super) const ARENA_HEIGHT: u32 = 11;
