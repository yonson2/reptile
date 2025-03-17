use crate::game::constants::*;
use bevy::prelude::*;

use super::{Score, ScoreboardUi};
pub mod menu;

pub(super) fn setup_scoreboard(mut commands: Commands) {
    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));
}

pub(super) fn cleanup_scoreboard(
    mut commands: Commands,
    scoreboard: Single<Entity, With<ScoreboardUi>>,
) {
    commands.entity(scoreboard.into_inner()).despawn_recursive();
}
