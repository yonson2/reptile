use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

// Constants
const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

// Components
#[derive(Component)]
struct SnakeHead;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

// Systems
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn((Sprite {
            color: SNAKE_HEAD_COLOR,
            ..default()
        },))
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn snake_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in &mut head_positions {
        if keys.pressed(KeyCode::ArrowLeft) {
            pos.x -= 1;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            pos.x += 1;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            pos.y += 1;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            pos.y -= 1;
        }

        // Wrap around logic
        if pos.x < 0 {
            pos.x = ARENA_WIDTH as i32 - 1;
        } else if pos.x >= ARENA_WIDTH as i32 {
            pos.x = 0;
        }

        if pos.y < 0 {
            pos.y = ARENA_HEIGHT as i32 - 1;
        } else if pos.y >= ARENA_HEIGHT as i32 {
            pos.y = 0;
        }
    }
}

fn size_scaling(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_scale: Query<(&Size, &mut Transform)>,
) {
    let window = q_window.single();
    for (sprite_size, mut transform) in &mut q_scale {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
            1.0,
        )
    }
}

fn position_translation(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = q_window.single();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1000., 1000.).with_scale_factor_override(1.0),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, (setup_camera, spawn_snake).chain())
        .add_systems(Update, snake_movement)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}
