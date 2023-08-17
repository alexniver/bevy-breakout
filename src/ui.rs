use bevy::prelude::*;

use crate::{
    game_state::GameState,
    score::{Score, ScoreChangeEvent},
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, refresh_score)
            .add_systems(OnEnter(GameState::GameOver), enter_gameover)
            .add_systems(OnExit(GameState::GameOver), exit_gameover);
    }
}

#[derive(Debug, Component)]
struct ScoreText;
#[derive(Debug, Component)]
struct GameoverText;

fn setup(mut commands: Commands) {
    let mut text_bundle = TextBundle::from_sections([TextSection::new(
        "Game Over, press r restart",
        TextStyle {
            font_size: 70.0,
            color: Color::ORANGE_RED,
            ..default()
        },
    )])
    .with_style(Style {
        position_type: PositionType::Absolute,
        width: Val::Px(1600.),
        height: Val::Px(160.),
        top: Val::Percent(0.5),
        left: Val::Percent(30.),
        ..default()
    });
    text_bundle.visibility = Visibility::Hidden;

    commands.spawn((text_bundle, GameoverText));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 100.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 100.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(100.0),
            ..default()
        }),
        ScoreText,
    ));
}

fn refresh_score(
    mut query_score: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
    mut score_add_event_reader: EventReader<ScoreChangeEvent>,
) {
    if score_add_event_reader.is_empty() {
        return;
    }
    score_add_event_reader.clear();

    let mut text = query_score.single_mut();
    text.sections[1].value = format!("{}", score.0);
}

fn enter_gameover(mut visibility_query: Query<&mut Visibility, With<GameoverText>>) {
    let mut visibility = visibility_query.single_mut();
    *visibility = Visibility::Visible;
}

fn exit_gameover(mut visibility_query: Query<&mut Visibility, With<GameoverText>>) {
    let mut visibility = visibility_query.single_mut();
    *visibility = Visibility::Hidden;
}
