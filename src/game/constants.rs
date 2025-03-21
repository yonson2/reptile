use bevy::{color::Color, ui::Val};

// Constants
//opens mouth at +1/2/3/4 (bigger each) for each direciton

pub(super) const SPRITE_PIXEL_SIZE: f32 = 16.;

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

pub(super) const FOOD_RED: usize = 336;
pub(super) const FOOD_GREEN: usize = 337;
pub(super) const FOOD_YELLOW: usize = 338;

pub(super) const SCOREBOARD_FONT_SIZE: f32 = 33.0;
pub(super) const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub(super) const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
pub(super) const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub(super) const CONTROLLER_UP: usize = 0;
pub(super) const CONTROLLER_HALF_UP: usize = 1;
pub(super) const CONTROLLER_PRESSED_UP: usize = 2;

pub(super) const CONTROLLER_DOWN: usize = 9;
pub(super) const CONTROLLER_HALF_DOWN: usize = 10;
pub(super) const CONTROLLER_PRESSED_DOWN: usize = 11;

pub(super) const CONTROLLER_LEFT: usize = 6;
pub(super) const CONTROLLER_HALF_LEFT: usize = 7;
pub(super) const CONTROLLER_PRESSED_LEFT: usize = 8;

pub(super) const CONTROLLER_RIGHT: usize = 3;
pub(super) const CONTROLLER_HALF_RIGHT: usize = 4;
pub(super) const CONTROLLER_PRESSED_RIGHT: usize = 5;

// pub(super) const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

//TODO: this is closely related to the window resolution if we don't plan on resizing
// think about this.
pub const ARENA_WIDTH: u32 = 8;
pub const ARENA_HEIGHT: u32 = 16;
