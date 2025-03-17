use bevy::prelude::*;

use crate::game::{constants::*, AppState, Score};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct GameOverScreen;

//TODO: rename to setup
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

type InteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
>;

pub fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: InteractionQuery,
    mut keys: ResMut<ButtonInput<KeyCode>>,
) {
    for (interaction, mut color) in &mut interaction_query {
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
                Text::new("(Press Up to play again)"),
                TextFont {
                    font_size: 33.,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}
