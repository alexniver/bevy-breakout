use bevy::prelude::*;

use crate::score::{Score, ScoreAddEvent};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, refresh_score);
    }
}

#[derive(Debug, Component)]
struct ScoreText;
#[derive(Debug, Component)]
struct GameoverText;

fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_sections([TextSection::new(
                    "Game Over, press r restart",
                    TextStyle {
                        font_size: 90.0,
                        color: Color::GRAY,
                        ..default()
                    },
                )])
                .with_style(Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(1600.),
                    height: Val::Px(160.),
                    ..default()
                }),
                GameoverText,
            ));
            builder.spawn((
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
        });
}

fn refresh_score(
    mut query_score: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
    mut score_add_event_reader: EventReader<ScoreAddEvent>,
) {
    if score_add_event_reader.is_empty() {
        return;
    }
    score_add_event_reader.clear();

    let mut text = query_score.single_mut();
    text.sections[1].value = format!("{}", score.0);
}
