use bevy::prelude::*;

#[derive(Debug, Resource, Default, States, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    BallIdle,
    BallFly,
    GameOver,
}
