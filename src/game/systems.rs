use crate::game::components::*;
use crate::game::constants::*;
use crate::game::events::*;
use crate::game::resources::*;

use bevy::prelude::*;
use bevy::{audio::PlaybackMode, ecs::system::SystemParam, window::PrimaryWindow};
use world::GameState;

use crate::assets::{AudioAsset, SnakeAsset};

pub mod world;

pub(super) type Either<T, U> = Or<(With<T>, With<U>)>;

#[derive(Component)]
pub struct MainGameScreen;

pub(super) fn setup_game(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut segments: ResMut<SnakeSegments>,
    mut food_writer: EventWriter<FoodEvent>,
    snake_asset: Res<SnakeAsset>,
) {
    // We cleanup the score here because we also use it
    // when we have finished the game so game destructors
    // would kill that info.
    score.0 = 0;
    //setup scoreboard
    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            MainGameScreen,
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::new("0"),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));

    commands
        .spawn(Direction::default())
        .insert(UserInput)
        .insert(MainGameScreen);

    *segments = SnakeSegments(vec![
        commands
            .spawn(Sprite::from_atlas_image(
                snake_asset.0.texture.clone(),
                TextureAtlas {
                    layout: snake_asset.0.atlas_layout.clone(),
                    index: SNAKE_HEAD_UP,
                },
            ))
            .insert(ImageAsset)
            .insert(MainGameScreen)
            .insert(SnakeHead)
            .insert(Direction::default())
            .insert(Position { x: 5, y: 5 })
            .insert(Size::square(1.))
            .id(),
        spawn_snake_segment(
            commands,
            Position { x: 5, y: 4 },
            snake_asset,
            SNAKE_TAIL_UP,
        ),
    ]);

    food_writer.send(FoodEvent);
}

pub(super) fn spawn_snake_segment(
    mut commands: Commands,
    position: Position,
    snake_asset: Res<SnakeAsset>,
    sprite_index: usize,
) -> Entity {
    commands
        .spawn(Sprite::from_atlas_image(
            snake_asset.0.texture.clone(),
            TextureAtlas {
                layout: snake_asset.0.atlas_layout.clone(),
                index: sprite_index,
            },
        ))
        .insert(MainGameScreen)
        .insert(ImageAsset)
        .insert(SnakeBody)
        .insert(position)
        .insert(Size::square(1.))
        .id()
}

pub(super) fn spawn_food(mut commands: Commands, position: Position, snake_asset: Res<SnakeAsset>) {
    // Randomly choose between the three food colors
    let food_index = match fastrand::u8(0..3) {
        0 => FOOD_RED,
        1 => FOOD_GREEN,
        _ => FOOD_YELLOW,
    };

    commands
        .spawn(Sprite::from_atlas_image(
            snake_asset.0.texture.clone(),
            TextureAtlas {
                layout: snake_asset.0.atlas_layout.clone(),
                index: food_index,
            },
        ))
        .insert(MainGameScreen)
        .insert(ImageAsset)
        .insert(Food)
        .insert(position)
        .insert(Size::square(1.));
}

pub(super) fn spawn_food_empty_position(
    commands: Commands,
    positions: Query<&Position>,
    food: Query<&Position, With<Food>>,
    mut food_reader: EventReader<FoodEvent>,
    snake_asset: Res<SnakeAsset>,
) {
    if food_reader.read().next().is_some() && food.iter().count() == 0 {
        let mut new_food_position;
        'outer: loop {
            new_food_position = Position {
                x: (fastrand::f32() * ARENA_WIDTH as f32) as i32,
                y: (fastrand::f32() * ARENA_HEIGHT as f32) as i32,
            };

            for &pos in &positions {
                if new_food_position == pos {
                    continue 'outer;
                }
            }
            break;
        }
        spawn_food(commands, new_food_position, snake_asset);
    }
}

pub(super) fn snake_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    input_direction: Single<&mut Direction, (With<UserInput>, Without<SnakeHead>)>,
    snake_dir: Option<Single<&Direction, With<SnakeHead>>>,
) {
    let mut input_direction = input_direction.into_inner();

    if snake_dir.is_none() {
        return;
    }

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

    if new_dir != snake_dir.unwrap().opposite() {
        *input_direction = new_dir;
    }
}

pub(super) fn snake_repaint(
    segments: Res<SnakeSegments>,
    positions: Query<&Position, Either<SnakeHead, SnakeBody>>,
    foods: Query<&Position, With<Food>>,
    mut sprites: Query<&mut Sprite, Either<SnakeHead, SnakeBody>>,
    head_dir: Option<Single<&Direction, With<SnakeHead>>>,
) {
    if head_dir.is_none() {
        return;
    }

    let head_dir = head_dir.unwrap().into_inner();

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
            // Now, for each food, see if we are close to the head.
            for &food_pos in foods.iter() {
                let head_pos = *segment_positions.get(i).expect("a head");

                let index = match head_dir {
                    Direction::Down => SNAKE_HEAD_DOWN + open_mouth(food_pos, head_pos),
                    Direction::Up => SNAKE_HEAD_UP + open_mouth(food_pos, head_pos),
                    Direction::Left => SNAKE_HEAD_LEFT + open_mouth(food_pos, head_pos),
                    Direction::Right => SNAKE_HEAD_RIGHT + open_mouth(food_pos, head_pos),
                };
                sprite.texture_atlas.as_mut().unwrap().index = index;
            }
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

pub(super) fn snake_movement(
    segments: Res<SnakeSegments>,
    input_direction: Single<&mut Direction, (With<UserInput>, Without<SnakeHead>)>,
    head: Option<Single<(Entity, &mut Direction), With<SnakeHead>>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut Position>,
) {
    let mut input_direction = input_direction.into_inner();

    if head.is_none() {
        return;
    }

    let (head_entity, mut head_direction) = head.unwrap().into_inner();

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
        *input_direction = Direction::Up;
        next_state.set(GameState::GameOver);
        return;
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

pub(super) fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_pos: Query<(Entity, &Position), With<Food>>,
    head_pos: Option<Single<&Position, With<SnakeHead>>>,
) {
    if head_pos.is_none() {
        return;
    }
    let head_pos = head_pos.unwrap().into_inner();
    for (ent, food_pos) in &food_pos {
        if head_pos == food_pos {
            commands.entity(ent).despawn();
            growth_writer.send(GrowthEvent);
        }
    }
}

// Group related resources for snake growth
#[derive(SystemParam)]
pub(super) struct SnakeGrowthParams<'w, 's> {
    last_tail_position: Res<'w, LastTailPosition>,
    head: Query<'w, 's, &'static Direction, With<SnakeHead>>,
    segments: ResMut<'w, SnakeSegments>,
    growth_reader: EventReader<'w, 's, GrowthEvent>,
    food_writer: EventWriter<'w, FoodEvent>,
    snake_asset: Res<'w, SnakeAsset>,
    audio: Res<'w, AudioAsset>,
    score: ResMut<'w, Score>,
    score_root: Single<'w, Entity, (With<ScoreboardUi>, With<Text>)>,
    writer: TextUiWriter<'w, 's>,
}

pub(super) fn snake_growth(mut commands: Commands, mut params: SnakeGrowthParams) {
    if params.growth_reader.read().next().is_some() {
        commands.spawn((
            AudioPlayer(params.audio.0.clone()),
            MainGameScreen,
            PlaybackSettings {
                mode: PlaybackMode::Despawn,
                paused: false,
                ..default()
            },
        ));
        let snake_direction = *params.head.single();
        let index = match snake_direction {
            Direction::Up => SNAKE_TAIL_UP,
            Direction::Down => SNAKE_TAIL_DOWN,
            Direction::Left => SNAKE_TAIL_LEFT,
            Direction::Right => SNAKE_TAIL_RIGHT,
        };
        params.segments.0.push(spawn_snake_segment(
            commands,
            params
                .last_tail_position
                .0
                .expect("last tail should be set when growing"),
            params.snake_asset,
            index,
        ));
        params.score.0 += 1;
        *params.writer.text(*params.score_root, 1) = params.score.0.to_string();
        params.food_writer.send(FoodEvent);
    }
}

pub(super) fn game_over_input(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.clear_just_pressed(KeyCode::ArrowUp) {
        next_state.set(GameState::Playing);
        keys.reset_all();
    }
}

// Right now I'm not inserting non-images
// TODO: refactor to use two different systems.
//
// To insert a non image, insert a sprite without
// a ImageAsset component.
// commands
//     .spawn((Sprite {
//         color: FOOD_COLOR,
//         ..default()
//     },))
//     .insert(Food)
//     .insert(position)
//     .insert(Size::square(0.8));
//
pub(super) fn size_scaling(
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    mut q_scale: Query<(&Size, &mut Transform, Option<&ImageAsset>)>,
) {
    if let Some(window) = window {
        let tile_width = window.width() / ARENA_WIDTH as f32;
        let tile_height = window.height() / ARENA_HEIGHT as f32;

        for (sprite_size, mut transform, is_image) in &mut q_scale {
            if is_image.is_some() {
                let sprite_pixel_size = SPRITE_PIXEL_SIZE;

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
}

pub(super) fn position_translation(
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    if let Some(window) = window {
        fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
            let tile_size = bound_window / bound_game;
            pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
        }
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
                0.0,
            );
        }
    }
}

fn open_mouth(food: Position, head: Position) -> usize {
    let manhattan_distance = (food.x - head.x).abs() + (food.y - head.y).abs();
    if manhattan_distance <= 4 {
        5 - manhattan_distance as usize
    } else {
        0
    }
}
