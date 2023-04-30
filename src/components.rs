use super::resources::*;

use bevy::prelude::*;

pub const X_BOTTOM: f32 = -250.0;
pub const Y_BOTTOM: f32 = -250.0;

pub const WINDOW_WIDTH: f32 = 500.0;
pub const WINDOW_HEIGHT: f32 = 500.0;

pub const GRID_SIZE: f32 = WINDOW_WIDTH / GRID_X_LENGTH as f32;

#[derive(Component)]
pub struct Player {}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    Move,
    #[default]
    Stop,
}

#[derive(Component)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn translation(&self)->Vec3{
        let x: f32 = self.x as f32 * GRID_SIZE + GRID_SIZE/2.0 + X_BOTTOM;
        let y: f32 = self.y as f32 * GRID_SIZE + GRID_SIZE/2.0 + Y_BOTTOM;

        return Vec3::new(x, y, 0.0);
    }
}

#[derive(Component)]
pub struct MoveDirection {
    pub vec3: Vec3,
}