use bevy::prelude::*;

use crate::{brick::BrickDespawnEvent, game_state::GameState};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreChangeEvent>()
            .insert_resource(Score(0))
            .add_systems(OnEnter(GameState::BallIdle), score_zero)
            .add_systems(Update, score_add);
    }
}

#[derive(Debug, Resource)]
pub struct Score(pub u32);

#[derive(Debug, Event)]
pub struct ScoreChangeEvent;

fn score_zero(
    mut score: ResMut<Score>,
    mut score_change_event_writer: EventWriter<ScoreChangeEvent>,
) {
    score.0 = 0;
    score_change_event_writer.send(ScoreChangeEvent);
}

fn score_add(
    mut score: ResMut<Score>,
    mut brick_despawn_event_reader: EventReader<BrickDespawnEvent>,
    mut score_change_event_writer: EventWriter<ScoreChangeEvent>,
) {
    if brick_despawn_event_reader.is_empty() {
        return;
    }
    brick_despawn_event_reader.clear();
    score.0 += 1;
    score_change_event_writer.send(ScoreChangeEvent);
}
