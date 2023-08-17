use bevy::{prelude::*, window::close_on_esc};
use bevy_breakout::{
    ball::BallPlugin,
    brick::BrickPlugin,
    game_state::{GameState, GameStatePlugin},
    paddle::PaddlePlugin,
    score::ScorePlugin,
    ui::UiPlugin,
    wall::WallPlugin,
};

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(WallPlugin)
        .add_plugins(BrickPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(GameStatePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
