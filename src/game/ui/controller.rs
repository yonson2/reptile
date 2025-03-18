use crate::{
    assets::{ControllerAsset, ImageAsset},
    game::{
        constants::*,
        events::ControllerEvent,
        systems::world::{AppState, GameState},
        ArbitraryPosition, Controller, Direction, Height, MainGameScreen, Position, Size,
    },
};
use bevy::{input::touch::Touches, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_event::<ControllerEvent>()
        .add_systems(
            Update,
            setup_controller_if_needed
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), despawn_controller)
        .add_systems(
            Update,
            controller_mouse_input
                .run_if(in_state(GameState::Playing))
                .run_if(not(in_state(AppState::Loading))),
        )
        .add_systems(
            Update,
            controller_touch_input
                .run_if(in_state(GameState::Playing))
                .run_if(not(in_state(AppState::Loading))),
        )
        .add_systems(
            Update,
            handle_controller_events
                .run_if(in_state(GameState::Playing))
                .run_if(not(in_state(AppState::Loading))),
        );
}

fn despawn_controller(mut commands: Commands, query: Query<Entity, With<Controller>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Check if controllers already exist, if not, set them up
fn setup_controller_if_needed(
    mut commands: Commands,
    controller_asset: Res<ControllerAsset>,
    controller_query: Query<&Controller>,
) {
    // Only setup controllers if none exist yet
    if controller_query.is_empty() {
        setup_controller(&mut commands, controller_asset);
    }
}

fn setup_controller(commands: &mut Commands, controller_asset: Res<ControllerAsset>) {
    let controller_buttons = [
        (CONTROLLER_UP, 3.5, 2.75, Direction::Up),
        (CONTROLLER_DOWN, 3.5, 1.25, Direction::Down),
        (CONTROLLER_LEFT, 2.75, 2.0, Direction::Left),
        (CONTROLLER_RIGHT, 4.25, 2.0, Direction::Right),
    ];

    for (index, x, y, dir) in controller_buttons {
        spawn_controller_button(commands, &controller_asset, index, x, y, dir);
    }
}

/// Shared function to process a pointer position (from mouse or touch)
fn process_pointer_input(
    pointer_position: Vec2,
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    controller_sprites: &Query<(&GlobalTransform, &Direction, &Position), With<Controller>>,
    controller_events: &mut EventWriter<ControllerEvent>,
) {
    // Get the camera to convert screen coordinates to world coordinates
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, pointer_position) else {
        return;
    };

    let window_width = window.width();
    let window_height = window.height();

    let tile_width = window_width / ARENA_WIDTH as f32;
    let tile_height = window_height / ARENA_HEIGHT as f32;

    // Check if any controller sprite was activated
    for (transform, direction, _) in controller_sprites.iter() {
        let sprite_position = transform.translation().truncate();

        // Use a smaller hit area
        let half_size_x = tile_width * 0.35;
        let half_size_y = tile_height * 0.35;

        if world_position.x >= sprite_position.x - half_size_x
            && world_position.x <= sprite_position.x + half_size_x
            && world_position.y >= sprite_position.y - half_size_y
            && world_position.y <= sprite_position.y + half_size_y
        {
            controller_events.send(ControllerEvent {
                direction: *direction,
            });
        }
    }
}

/// System to handle controller button clicks
fn controller_mouse_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    controller_sprites: Query<(&GlobalTransform, &Direction, &Position), With<Controller>>,
    mut controller_events: EventWriter<ControllerEvent>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window_q.get_single().unwrap();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_q.get_single().unwrap();

    process_pointer_input(
        cursor_position,
        window,
        camera,
        camera_transform,
        &controller_sprites,
        &mut controller_events,
    );
}

/// System to handle controller button touch events
fn controller_touch_input(
    touches: Res<Touches>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    controller_sprites: Query<(&GlobalTransform, &Direction, &Position), With<Controller>>,
    mut controller_events: EventWriter<ControllerEvent>,
) {
    // Only process newly pressed touches
    for touch in touches.iter_just_pressed() {
        let window = window_q.get_single().unwrap();
        let (camera, camera_transform) = camera_q.get_single().unwrap();

        process_pointer_input(
            touch.position(),
            window,
            camera,
            camera_transform,
            &controller_sprites,
            &mut controller_events,
        );
    }
}

fn spawn_controller_button(
    commands: &mut Commands,
    controller_asset: &ControllerAsset,
    index: usize,
    x: f32,
    y: f32,
    dir: Direction,
) {
    commands
        .spawn(Sprite::from_atlas_image(
            controller_asset.0.texture.clone(),
            TextureAtlas {
                layout: controller_asset.0.atlas_layout.clone(),
                index,
            },
        ))
        .insert(dir)
        .insert(Height(1.))
        .insert(Controller)
        .insert(ImageAsset)
        .insert(MainGameScreen)
        .insert(Position::Arbitrary(ArbitraryPosition { x, y }))
        .insert(Size::square(1.));
}

#[derive(Component, Default)]
struct ButtonAnimationState {
    ///(0 = normal, 1 = half-pressed, 2 = fully-pressed)
    step: u8,
    timer: Timer,
    is_animating: bool,
}

fn handle_controller_events(
    mut commands: Commands,
    mut controller_events: EventReader<ControllerEvent>,
    mut query: Query<(Entity, &Direction, &mut Sprite), With<Controller>>,
    mut animation_query: Query<(Entity, &mut ButtonAnimationState)>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (entity, mut animation_state) in animation_query.iter_mut() {
        if animation_state.is_animating {
            animation_state.timer.tick(time.delta());

            if animation_state.timer.just_finished() {
                if let Ok((_, direction, mut sprite)) = query.get_mut(entity) {
                    animation_state.step += 1;

                    match direction {
                        Direction::Up => match animation_state.step {
                            1 => sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_UP,
                            2 => {
                                sprite.texture_atlas.as_mut().unwrap().index =
                                    CONTROLLER_PRESSED_UP;
                                keys.release(Direction::Up.to_keycode());
                            }
                            3 => sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_UP,
                            4 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_UP;
                                animation_state.is_animating = false;
                                animation_state.step = 0;
                            }
                            _ => {}
                        },
                        Direction::Down => match animation_state.step {
                            1 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_DOWN
                            }
                            2 => {
                                sprite.texture_atlas.as_mut().unwrap().index =
                                    CONTROLLER_PRESSED_DOWN;
                                keys.release(Direction::Down.to_keycode());
                            }
                            3 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_DOWN
                            }
                            4 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_DOWN;
                                animation_state.is_animating = false;
                                animation_state.step = 0;
                            }
                            _ => {}
                        },
                        Direction::Left => match animation_state.step {
                            1 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_LEFT
                            }
                            2 => {
                                sprite.texture_atlas.as_mut().unwrap().index =
                                    CONTROLLER_PRESSED_LEFT;
                                keys.release(Direction::Left.to_keycode());
                            }
                            3 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_LEFT
                            }
                            4 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_LEFT;
                                animation_state.is_animating = false;
                                animation_state.step = 0;
                            }
                            _ => {}
                        },
                        Direction::Right => match animation_state.step {
                            1 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_RIGHT
                            }
                            2 => {
                                sprite.texture_atlas.as_mut().unwrap().index =
                                    CONTROLLER_PRESSED_RIGHT;
                                keys.release(Direction::Right.to_keycode());
                            }
                            3 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_HALF_RIGHT
                            }
                            4 => {
                                sprite.texture_atlas.as_mut().unwrap().index = CONTROLLER_RIGHT;
                                animation_state.is_animating = false;
                                animation_state.step = 0;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
    }

    // Process new controller events
    for event in controller_events.read() {
        // Find the corresponding button entity
        for (entity, direction, _) in query.iter() {
            if *direction == event.direction {
                // Only press the key and don't release it immediately
                // This allows the input system to detect the keypress
                keys.press(direction.to_keycode());

                // Check if this entity already has an animation state
                if let Ok((_, mut animation_state)) = animation_query.get_mut(entity) {
                    // Reset existing animation state
                    animation_state.step = 0;
                    animation_state.timer = Timer::from_seconds(0.05, TimerMode::Repeating);
                    animation_state.is_animating = true;
                } else {
                    // Create new animation state for this button
                    commands.entity(entity).insert(ButtonAnimationState {
                        step: 0,
                        timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                        is_animating: true,
                    });
                }
            }
        }
    }
}
