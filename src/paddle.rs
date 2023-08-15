use bevy::prelude::*;

use crate::{collider::Collider, wall::WALL_WIDTH};

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_paddle);
    }
}

pub const SPEED: f32 = 800.0;
pub const PADDLE_WIDTH: f32 = 300.0;
pub const PADDLE_HEIGHT: f32 = 10.0;
pub const PADDLE_POS_Y: f32 = -300.0;

const MIN_X: f32 = -WALL_WIDTH / 2.0 + PADDLE_WIDTH / 2.0;
const MAX_X: f32 = -MIN_X;

#[derive(Debug, Component)]
pub struct Paddle;

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.9, 0.1),
                ..default()
            },
            //transform: Transform::from_xyz(0.0, PADDLE_POS_Y, 0.0),
            transform: Transform {
                translation: Vec3::new(0.0, PADDLE_POS_Y, 0.0),
                scale: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT).extend(1.0),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));
}

fn move_paddle(
    mut query_paddle: Query<&mut Transform, With<Paddle>>,
    keys: ResMut<Input<KeyCode>>,
    time_step: Res<FixedTime>,
) {
    let mut t = query_paddle.single_mut();
    let mut direction = 0.0_f32;
    if keys.pressed(KeyCode::A) {
        direction = -1.0;
    } else if keys.pressed(KeyCode::D) {
        direction = 1.0;
    }

    t.translation.x += direction * time_step.period.as_secs_f32() * SPEED;
    t.translation.x = t.translation.x.min(MAX_X).max(MIN_X);
}
