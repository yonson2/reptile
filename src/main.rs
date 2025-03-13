use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
    window::{PrimaryWindow, WindowResolution},
};
use rand::random;

// Constants
const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

// Components
#[derive(Component, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct Food;

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

// Resources
#[derive(Default, Resource)]
struct SnakeSegments(Vec<Entity>);

#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

// Events
#[derive(Event)]
struct GrowthEvent;

#[derive(Event)]
struct GameOverEvent;

// Systems
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            })
            .insert(SnakeHead)
            .insert(Direction::Up)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_snake_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

fn spawn_snake_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        })
        .insert(SnakeBody)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn spawn_food(mut commands: Commands) {
    commands
        .spawn((Sprite {
            color: FOOD_COLOR,
            ..default()
        },))
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn snake_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut snake_dirs: Query<&mut Direction, With<SnakeHead>>,
) {
    let mut dir = snake_dirs.single_mut();
    let new_dir = if keys.pressed(KeyCode::ArrowLeft) {
        Direction::Left
    } else if keys.pressed(KeyCode::ArrowRight) {
        Direction::Right
    } else if keys.pressed(KeyCode::ArrowDown) {
        Direction::Down
    } else if keys.pressed(KeyCode::ArrowUp) {
        Direction::Up
    } else {
        *dir
    };

    if new_dir != dir.opposite() {
        *dir = new_dir;
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    heads: Query<(Entity, &Direction), With<SnakeHead>>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut Position>,
) {
    let (head_entity, head_direction) = heads.single();

    // Make a copy of the previous positions.
    let segment_positions = segments
        .0
        .iter()
        .map(|e| {
            *positions
                .get_mut(*e)
                .expect("each body part should have a position")
        })
        .collect::<Vec<Position>>();

    // Update head.
    let mut head_pos = positions
        .get_mut(head_entity)
        .expect("snake head should exist");

    match *head_direction {
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
        Direction::Down => head_pos.y -= 1,
        Direction::Up => head_pos.y += 1,
    }

    // Uncomment this block and comment the next one to wrap-around the edges
    // if head_pos.x < 0 {
    //     head_pos.x = ARENA_WIDTH as i32 - 1;
    // } else if head_pos.x >= ARENA_WIDTH as i32 {
    //     head_pos.x = 0;
    // }
    //
    // if head_pos.y < 0 {
    //     head_pos.y = ARENA_HEIGHT as i32 - 1;
    // } else if head_pos.y >= ARENA_HEIGHT as i32 {
    //     head_pos.y = 0;
    // }

    if head_pos.x < 0
        || head_pos.y < 0
        || head_pos.x as u32 >= ARENA_WIDTH
        || head_pos.y as u32 >= ARENA_HEIGHT
    {
        game_over_writer.send(GameOverEvent);
    }

    if segment_positions.contains(&head_pos) {
        game_over_writer.send(GameOverEvent);
    }

    // Make rest of body follow its parent.
    segment_positions
        .iter()
        .zip(segments.0.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });

    *last_tail_position = LastTailPosition(segment_positions.last().copied());
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_pos: Query<(Entity, &Position), With<Food>>,
    head_pos: Query<&Position, With<SnakeHead>>,
) {
    let head_pos = head_pos.single();
    for (ent, food_pos) in &food_pos {
        if head_pos == food_pos {
            commands.entity(ent).despawn();
            growth_writer.send(GrowthEvent);
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        segments
            .0
            .push(spawn_snake_segment(commands, last_tail_position.0.unwrap()));
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeBody>>,
) {
    if reader.read().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
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
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_systems(Startup, (setup_camera, spawn_snake).chain())
        .add_systems(Update, snake_movement_input.before(snake_movement))
        .add_systems(
            Update,
            snake_movement.run_if(on_timer(Duration::from_secs_f32(0.15))),
        )
        .add_systems(
            Update,
            spawn_food.run_if(on_timer(Duration::from_secs_f32(1.))),
        )
        .add_systems(Update, snake_eating.after(snake_movement))
        .add_systems(Update, game_over.after(snake_movement))
        .add_systems(Update, snake_growth.after(snake_eating))
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .run();
}
