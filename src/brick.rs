use bevy::prelude::*;

use crate::{collider::Collider, game_state::GameState};

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BrickDespawnEvent>()
            .add_systems(OnEnter(GameState::BallIdle), (cleanup.before(setup), setup));
    }
}

#[derive(Component)]
pub struct Brick;

#[derive(Debug, Event)]
pub struct BrickDespawnEvent;

pub const BRICK_WIDTH: f32 = 10.0;
pub const BRICK_HEIGHT: f32 = 10.0;
pub const BRICK_PADDING: f32 = 1.0;
pub const BRICK_NUM_X: u32 = 60;
pub const BRICK_NUM_Y: u32 = 40;

fn setup(mut commands: Commands) {
    let start_pos = Vec2::new(-(BRICK_NUM_X as f32) * BRICK_WIDTH / 2.0, 0.0);
    let brick_size = Vec2::new(
        BRICK_WIDTH - BRICK_PADDING * 2.0,
        BRICK_HEIGHT - BRICK_PADDING * 2.0,
    );
    for x in 0..BRICK_NUM_X {
        for y in 0..BRICK_NUM_Y {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.8, 0.2),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            start_pos.x + x as f32 * BRICK_WIDTH + BRICK_WIDTH / 2.0,
                            start_pos.y + y as f32 * BRICK_HEIGHT + BRICK_HEIGHT / 2.0,
                            0.1,
                        ),
                        scale: brick_size.extend(1.0),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }
}

fn cleanup(mut commands: Commands, query_brick: Query<Entity, With<Brick>>) {
    for entity in query_brick.iter() {
        commands.entity(entity).despawn();
    }
}
