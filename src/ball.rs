use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};

use crate::{
    brick::{Brick, BrickDespawnEvent, BRICK_WIDTH},
    collider::Collider,
    game_state::GameState,
    paddle::{Paddle, PADDLE_HEIGHT},
    velocity::Velocity,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (follow_paddle, ball_start_fly).run_if(in_state(GameState::BallIdle)),
            )
            .add_systems(
                Update,
                (ball_flying, ball_collide).run_if(in_state(GameState::BallFly)),
            );
    }
}

const BALL_RADIS: f32 = BRICK_WIDTH / 2.0;
const BALL_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Ball;

fn setup(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshs.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform {
                scale: Vec3::new(BALL_RADIS * 2.0, BALL_RADIS * 2.0, 1.0),
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity::new(0.0, 0.0),
    ));
}

fn follow_paddle(
    query_paddle: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut query_ball: Query<&mut Transform, (With<Ball>, Without<Paddle>)>,
) {
    let mut t_ball = query_ball.single_mut();
    let t_paddle = query_paddle.single();
    t_ball.translation.x = t_paddle.translation.x;
    t_ball.translation.y = t_paddle.translation.y + BALL_RADIS + PADDLE_HEIGHT / 2.0;
}

fn ball_start_fly(
    keys: Res<Input<KeyCode>>,
    mut query_ball: Query<&mut Velocity, With<Ball>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut velocity = query_ball.single_mut();
        velocity.v.x = 1.0;
        velocity.v.y = 1.0;

        state.set(GameState::BallFly);
    }
}

fn ball_flying(
    mut query_ball: Query<(&mut Transform, &Velocity), With<Ball>>,
    time: Res<FixedTime>,
    mut state: ResMut<NextState<GameState>>,
) {
    let (mut t, velocity) = query_ball.single_mut();
    t.translation.x += velocity.v.x * time.period.as_secs_f32() * BALL_SPEED;
    t.translation.y += velocity.v.y * time.period.as_secs_f32() * BALL_SPEED;

    if t.translation.y < -500.0 {
        state.set(GameState::GameOver);
    }
}

fn ball_collide(
    mut commands: Commands,
    mut query_ball: Query<(&Transform, &mut Velocity), With<Ball>>,
    mut query_collider: Query<
        (Entity, &Transform, Option<&Brick>, Option<&Paddle>),
        With<Collider>,
    >,
    mut brick_despawn_event_writer: EventWriter<BrickDespawnEvent>,
) {
    let (t_ball, mut velocity) = query_ball.single_mut();
    let ball_pos = t_ball.translation;
    let ball_size = t_ball.scale.truncate();

    // ball collide with brick
    for (entity, t_collider, option_brick, option_paddle) in query_collider.iter_mut() {
        let collider_pos = t_collider.translation;
        let collider_size = t_collider.scale.truncate();
        if let Some(collision) = collide(ball_pos, ball_size, collider_pos, collider_size) {
            if let Some(_) = option_brick {
                commands.entity(entity).despawn();
                brick_despawn_event_writer.send(BrickDespawnEvent);
            }

            match collision {
                bevy::sprite::collide_aabb::Collision::Left => velocity.v.x *= -1.0,
                bevy::sprite::collide_aabb::Collision::Right => velocity.v.x *= -1.0,
                bevy::sprite::collide_aabb::Collision::Top => velocity.v.y *= -1.0,
                bevy::sprite::collide_aabb::Collision::Bottom => velocity.v.y *= -1.0,
                bevy::sprite::collide_aabb::Collision::Inside => {}
            }

            if let Some(paddle) = option_paddle {
                velocity.v.x += paddle.0 * 0.1;
            }
            return;
        }
    }
}
