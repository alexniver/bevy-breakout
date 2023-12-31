use bevy::prelude::*;

use crate::{brick::*, collider::Collider};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub const WALL_WIDTH: f32 = BRICK_WIDTH * BRICK_NUM_X as f32 + WALL_THICKNESS * 2.0;
const WALL_HIGHT: f32 = BRICK_HEIGHT * (BRICK_NUM_Y + 1) as f32 * 2.0;
const WALL_THICKNESS: f32 = 20.0;

#[derive(Component)]
pub struct Wall {
    pub pos: Vec2,
    pub size: Vec2,
}
impl Wall {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }
}

fn setup(mut commands: Commands) {
    let pos_x = WALL_WIDTH / 2.0;
    // left wall
    let pos = Vec2::new(-pos_x, 0.0);
    let size = Vec2::new(WALL_THICKNESS, WALL_HIGHT + WALL_THICKNESS);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                scale: size.extend(1.0),
                ..default()
            },
            ..default()
        },
        Wall::new(pos, size),
        Collider,
    ));

    // right wall
    let pos = Vec2::new(pos_x, 0.0);
    let size = Vec2::new(WALL_THICKNESS, WALL_HIGHT + WALL_THICKNESS);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                scale: size.extend(1.0),
                ..default()
            },
            ..default()
        },
        Wall::new(pos, size),
        Collider,
    ));

    // top wall
    let pos = Vec2::new(0.0, WALL_HIGHT / 2.0 + BRICK_HEIGHT);
    let size = Vec2::new(WALL_WIDTH + BRICK_WIDTH * 2.0, WALL_THICKNESS);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                scale: size.extend(1.0),
                ..default()
            },
            ..default()
        },
        Wall::new(pos, size),
        Collider,
    ));
}
