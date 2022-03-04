use bevy::prelude::*;

pub(crate) const TIME_STEP: f32 = 1.0 / 60.0;

pub(crate) const PLAYER_COLOR: [Color; 3] = [
    Color::rgb(1.0, 0., 0.), // Red
    Color::rgb(0., 1.0, 0.), // Green
    Color::rgb(0., 0., 1.0), // blue
];

pub(crate) const ARENA_WIDTH: u32 = 10;
pub(crate) const ARENA_HEIGHT: u32 = 10;

pub(crate) const CREATURE_TYPE: [&str; 3] = [
    "enemies/blue_stone.png",
    "enemies/green_stone.png",
    "enemies/brown_stone.png",
];
