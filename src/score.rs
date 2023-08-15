use bevy::prelude::*;

use crate::brick::BrickDespawnEvent;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreAddEvent>()
            .insert_resource(Score(0))
            .add_systems(Update, score_add);
    }
}

#[derive(Debug, Resource)]
pub struct Score(pub u32);

#[derive(Debug, Event)]
pub struct ScoreAddEvent;

fn score_add(
    mut score: ResMut<Score>,
    mut brick_despawn_event_reader: EventReader<BrickDespawnEvent>,
    mut score_add_event_writer: EventWriter<ScoreAddEvent>,
) {
    if brick_despawn_event_reader.is_empty() {
        return;
    }
    brick_despawn_event_reader.clear();
    score.0 += 1;
    score_add_event_writer.send(ScoreAddEvent);
}
