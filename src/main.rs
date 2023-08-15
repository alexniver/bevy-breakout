use bevy::{prelude::*, window::close_on_esc};
use bevy_breakout::{
    ball::BallPlugin, brick::BrickPlugin, game_state::GameState, paddle::PaddlePlugin,
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
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
