use bevy::{input::touch::Touches, prelude::*};

use crate::game::{constants::*, AppState, Score};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct GameOverScreen;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/fibberish.ttf"); //TODO: use handle from
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    top: Val::Px(150.),
                    ..default()
                },
                ..default()
            },
            MainMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Snake"),
                TextFont {
                    font: font.clone(),
                    font_size: 180.,
                    ..default()
                },
                TextColor(SCORE_COLOR),
            ));
        });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(40.),
                        height: Val::Percent(10.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font: font.clone(),
                            //assets.
                            font_size: 80.,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

type ButtonQuery<'a, 'b> = Query<
    'a,
    'b,
    (
        Entity,
        &'static GlobalTransform,
        &'static mut BackgroundColor,
    ),
    With<Button>,
>;

/// Updated menu function to handle both mouse clicks and touch events
pub fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut button_query: ButtonQuery,
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
) {
    // Process standard interaction events (for desktop/mouse hover effects)
    for (interaction, entity) in &interaction_query {
        if let Ok((_, _, mut color)) = button_query.get_mut(entity) {
            match *interaction {
                Interaction::Pressed => {
                    *color = NORMAL_BUTTON.into();
                    keys.reset_all();
                    next_state.set(AppState::Game);
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }

    // Get window and camera data
    let Ok(window) = window_q.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };

    // Handle touch input
    for touch in touches.iter_just_pressed() {
        process_button_input(
            touch.position(),
            window,
            camera,
            camera_transform,
            &mut button_query,
            &mut next_state,
            &mut keys,
        );
    }
}

/// Shared function to process a pointer position (from mouse or touch)
fn process_button_input(
    pointer_position: Vec2,
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    button_query: &mut ButtonQuery,
    next_state: &mut ResMut<NextState<AppState>>,
    keys: &mut ResMut<ButtonInput<KeyCode>>,
) {
    // Convert screen coordinates to world coordinates
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, pointer_position) else {
        return;
    };

    // Check if any button was clicked/touched
    for (_, transform, _) in button_query.iter_mut() {
        let button_position = transform.translation().truncate();

        // Get button size (assuming the UI calculation based on the viewport size)
        // Using a reasonable hit area for the button
        let button_width = window.width() * 0.4; // 40% of window width as in the button definition
        let button_height = window.height() * 0.1; // 10% of window height as in the button definition

        let half_width = button_width * 0.5;
        let half_height = button_height * 0.5;

        if world_position.x >= button_position.x - half_width
            && world_position.x <= button_position.x + half_width
            && world_position.y >= button_position.y - half_height
            && world_position.y <= button_position.y + half_height
        {
            keys.reset_all();
            next_state.set(AppState::Game);
        }
    }
}

pub fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let font = asset_server.load("fonts/fibberish.ttf"); //TODO: use handle from

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    top: Val::Px(250.),
                    ..default()
                },
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Game Over!"),
                TextFont {
                    font: font.clone(),
                    font_size: 130.,
                    ..default()
                },
                TextColor(SCORE_COLOR),
            ));
        });

    // Create a single container for all text elements
    let mut play_again_text = "(Press Up to play again)";
    #[cfg(target_arch = "wasm32")]
    {
        play_again_text = "(Reload to play again)";
    }
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|parent| {
            // Score text
            parent.spawn((
                Text::new(format!("Your score: {}", score.0)),
                TextFont {
                    font_size: 80.,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // Play again text
            parent.spawn((
                Text::new(play_again_text),
                TextFont {
                    font_size: 33.,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}
