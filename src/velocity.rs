use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Velocity {
    pub v: Vec2,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { v: Vec2::new(x, y) }
    }
}
