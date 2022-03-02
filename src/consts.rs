use bevy::prelude::*;

pub(crate) const TIME_STEP: f32 = 1.0 / 60.0;

pub(crate) const BOX_COLOR: [Color; 3] = [
    Color::rgb(1., 0., 0.),
    Color::rgb(0., 1., 0.),
    Color::rgb(0., 0., 1.),
];

pub(crate) const ARENA_WIDTH: u32 = 10;
pub(crate) const ARENA_HEIGHT: u32 = 10;
