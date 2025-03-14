use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use rand::random;

use crate::assets::GameAssets;

type Either<T, U> = Or<(With<T>, With<U>)>;

// Constants
const SNAKE_HEAD_UP: usize = 48;
const SNAKE_HEAD_DOWN: usize = 80;
const SNAKE_HEAD_LEFT: usize = 64;
const SNAKE_HEAD_RIGHT: usize = 96;

const SNAKE_BODY_VERTICAL: usize = 32;
const SNAKE_BODY_HORIZONTAL: usize = 33;

const SNAKE_CORNER_BOTTOM_RIGHT: usize = 34;
const SNAKE_CORNER_BOTTOM_LEFT: usize = 35;
const SNAKE_CORNER_TOP_RIGHT: usize = 36;
const SNAKE_CORNER_TOP_LEFT: usize = 37;

const SNAKE_TAIL_UP: usize = 38;
const SNAKE_TAIL_DOWN: usize = 40;
const SNAKE_TAIL_LEFT: usize = 39;
const SNAKE_TAIL_RIGHT: usize = 41;

const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

const ARENA_WIDTH: u32 = 11;
const ARENA_HEIGHT: u32 = 11;

// Components
#[derive(Component, Resource, PartialEq, Copy, Clone)]
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

#[derive(Component)]
struct ImageAsset;

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

#[derive(Event)]
struct FoodEvent;

fn spawn_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    mut food_writer: EventWriter<FoodEvent>,
    game_assets: Res<GameAssets>,
) {
    *segments = SnakeSegments(vec![commands
        .spawn(Sprite::from_atlas_image(
            game_assets.texture.clone(),
            TextureAtlas {
                layout: game_assets.atlas_layout.clone(),
                index: SNAKE_HEAD_UP,
            },
        ))
        .insert(ImageAsset)
        .insert(SnakeHead)
        .insert(Direction::Up)
        .insert(Position { x: 5, y: 5 })
        .insert(Size::square(1.))
        .id()]);

    food_writer.send(FoodEvent);
}

fn spawn_snake_segment(
    mut commands: Commands,
    position: Position,
    game_assets: Res<GameAssets>,
    sprite_index: usize,
) -> Entity {
    commands
        .spawn(Sprite::from_atlas_image(
            game_assets.texture.clone(),
            TextureAtlas {
                layout: game_assets.atlas_layout.clone(),
                index: sprite_index,
            },
        ))
        .insert(ImageAsset)
        .insert(SnakeBody)
        .insert(position)
        .insert(Size::square(1.))
        .id()
}

fn spawn_food(mut commands: Commands, position: Position) {
    commands
        .spawn((Sprite {
            color: FOOD_COLOR,
            ..default()
        },))
        .insert(Food)
        .insert(position)
        .insert(Size::square(0.8));
}

fn spawn_food_empty_position(
    commands: Commands,
    positions: Query<&Position>,
    mut food_reader: EventReader<FoodEvent>,
) {
    if food_reader.read().next().is_some() {
        let mut new_food_position;
        'outer: loop {
            new_food_position = Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            };

            for &pos in &positions {
                if new_food_position == pos {
                    continue 'outer;
                }
            }
            break;
        }
        spawn_food(commands, new_food_position);
    }
}

fn snake_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input_direction: ResMut<Direction>,
    snake_dir: Query<&Direction, With<SnakeHead>>,
) {
    let new_dir = if keys.pressed(KeyCode::ArrowLeft) {
        Direction::Left
    } else if keys.pressed(KeyCode::ArrowRight) {
        Direction::Right
    } else if keys.pressed(KeyCode::ArrowDown) {
        Direction::Down
    } else if keys.pressed(KeyCode::ArrowUp) {
        Direction::Up
    } else {
        *input_direction
    };

    if new_dir != snake_dir.single().opposite() {
        *input_direction = new_dir;
    }
}

fn snake_repaint(
    segments: Res<SnakeSegments>,
    positions: Query<&Position, Either<SnakeHead, SnakeBody>>,
    mut sprites: Query<&mut Sprite, Either<SnakeHead, SnakeBody>>,
    head_direction: Query<&Direction, With<SnakeHead>>,
) {
    let head_direction = *head_direction.single();
    let segment_positions = segments
        .0
        .iter()
        .map(|e| {
            *positions
                .get(*e)
                .expect("each body part should have a position")
        })
        .collect::<Vec<Position>>();

    // Helper function to determine the relative direction between two positions
    // accounting for arena wrapping
    fn get_direction(from: &Position, to: &Position) -> (i32, i32) {
        let mut dx = to.x - from.x;
        let mut dy = to.y - from.y;

        if dx > 1 {
            dx = -1; // Wrapped from right to left
        } else if dx < -1 {
            dx = 1; // Wrapped from left to right
        }

        if dy > 1 {
            dy = -1; // Wrapped from bottom to top
        } else if dy < -1 {
            dy = 1; // Wrapped from top to bottom
        }

        (dx, dy)
    }

    for (i, &entity) in segments.0.iter().enumerate() {
        let mut sprite = sprites.get_mut(entity).unwrap();
        // Head
        if i == 0 {
            let index = match head_direction {
                Direction::Down => SNAKE_HEAD_DOWN,
                Direction::Left => SNAKE_HEAD_LEFT,
                Direction::Up => SNAKE_HEAD_UP,
                Direction::Right => SNAKE_HEAD_RIGHT,
            };
            sprite.texture_atlas.as_mut().unwrap().index = index;
        }
        // Tail
        else if i == segments.0.len() - 1 {
            if let Some(&prev) = segment_positions.get(i - 1) {
                let tail = segment_positions[i];
                let (dx, dy) = get_direction(&tail, &prev);

                if dx > 0 {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_TAIL_RIGHT;
                } else if dx < 0 {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_TAIL_LEFT;
                } else if dy > 0 {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_TAIL_UP;
                } else {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_TAIL_DOWN;
                }
            }
        }
        // Body segments
        else {
            let prev = segment_positions[i - 1]; // Segment before this one
            let next = segment_positions[i + 1]; // Segment after this one
            let current = segment_positions[i]; // Current segment

            // Get directions accounting for wrapping
            let (prev_dx, prev_dy) = get_direction(&current, &prev);
            let (next_dx, next_dy) = get_direction(&current, &next);

            // Determine if this is a straight segment or a corner
            let is_horizontal = prev_dy == 0 && next_dy == 0;
            let is_vertical = prev_dx == 0 && next_dx == 0;

            if is_horizontal {
                sprite.texture_atlas.as_mut().unwrap().index = SNAKE_BODY_HORIZONTAL;
            } else if is_vertical {
                sprite.texture_atlas.as_mut().unwrap().index = SNAKE_BODY_VERTICAL;
            } else {
                // This is a corner piece - determine which corner based on directions
                if (prev_dx < 0 && next_dy < 0) || (prev_dy < 0 && next_dx < 0) {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_CORNER_TOP_RIGHT;
                } else if (prev_dx > 0 && next_dy < 0) || (prev_dy < 0 && next_dx > 0) {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_CORNER_TOP_LEFT;
                } else if (prev_dx < 0 && next_dy > 0) || (prev_dy > 0 && next_dx < 0) {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_CORNER_BOTTOM_RIGHT;
                } else {
                    sprite.texture_atlas.as_mut().unwrap().index = SNAKE_CORNER_BOTTOM_LEFT;
                }
            }
        }
    }
}

fn snake_movement(
    segments: Res<SnakeSegments>,
    input_direction: Res<Direction>,
    mut heads: Query<(Entity, &mut Direction), With<SnakeHead>>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut Position>,
) {
    let (head_entity, mut head_direction) = heads.single_mut();

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

    *head_direction = *input_direction;

    // Update head.
    let mut head_pos = positions
        .get_mut(head_entity)
        .expect("snake head should exist");

    match *head_direction {
        Direction::Left => {
            head_pos.x -= 1;
        }
        Direction::Right => {
            head_pos.x += 1;
        }
        Direction::Down => {
            head_pos.y -= 1;
        }
        Direction::Up => {
            head_pos.y += 1;
        }
    }

    if head_pos.x < 0 {
        head_pos.x = ARENA_WIDTH as i32 - 1;
    } else if head_pos.x >= ARENA_WIDTH as i32 {
        head_pos.x = 0;
    }

    if head_pos.y < 0 {
        head_pos.y = ARENA_HEIGHT as i32 - 1;
    } else if head_pos.y >= ARENA_HEIGHT as i32 {
        head_pos.y = 0;
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
    head: Query<&Direction, With<SnakeHead>>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    mut food_writer: EventWriter<FoodEvent>,
    game_assets: Res<GameAssets>,
) {
    if growth_reader.read().next().is_some() {
        let snake_direction = *head.single();
        let index = match snake_direction {
            Direction::Up => SNAKE_TAIL_UP,
            Direction::Down => SNAKE_TAIL_DOWN,
            Direction::Left => SNAKE_TAIL_LEFT,
            Direction::Right => SNAKE_TAIL_RIGHT,
        };
        segments.0.push(spawn_snake_segment(
            commands,
            last_tail_position
                .0
                .expect("last tail should be set when growing"),
            game_assets,
            index,
        ));
        food_writer.send(FoodEvent);
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    food_writer: EventWriter<FoodEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, Either<SnakeHead, SnakeBody>>,
    game_assets: Res<GameAssets>,
) {
    if reader.read().next().is_some() {
        // Despawn food, snake body segments, and the snake head
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res, food_writer, game_assets);
    }
}

fn size_scaling(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_scale: Query<(&Size, &mut Transform, Option<&ImageAsset>)>,
) {
    let window = q_window.single();
    let tile_width = window.width() / ARENA_WIDTH as f32;
    let tile_height = window.height() / ARENA_HEIGHT as f32;

    for (sprite_size, mut transform, is_image) in &mut q_scale {
        if is_image.is_some() {
            let sprite_pixel_size = 16.0; // Size of one sprite in the atlas

            transform.scale = Vec3::new(
                tile_width * sprite_size.width / sprite_pixel_size,
                tile_height * sprite_size.height / sprite_pixel_size,
                1.0,
            );
        } else {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width(),
                sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
                1.0,
            );
        }
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
