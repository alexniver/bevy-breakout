use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, restart.run_if(in_state(GameState::GameOver)));
    }
}

#[derive(Debug, Resource, Default, States, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    BallIdle,
    BallFly,
    GameOver,
}

pub fn restart(mut state: ResMut<NextState<GameState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::R) {
        state.set(GameState::BallIdle);
    }
}
