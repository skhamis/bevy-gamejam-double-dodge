use bevy::prelude::*;

pub(crate) const TIME_STEP: f32 = 1.0 / 60.0;

pub(crate) const PLAYER_COLOR: [Color; 3] = [
    Color::rgb(1.0, 0.07, 0.31),  // Player 1
    Color::rgb(0.04, 0.03, 0.03), // Wall
    Color::rgb(0., 0.57, 0.68),   // Player 2
];

// pub(crate) const ARENA_WIDTH: u32 = 10;
// pub(crate) const ARENA_HEIGHT: u32 = 10;

// pub(crate) const CREATURE_TYPE: [&str; 3] = [
//     "enemies/blue_stone.png",
//     "enemies/green_stone.png",
//     "enemies/brown_stone.png",
// ];

pub(crate) const CREATURE_COLORS: [Color; 3] = [
    Color::rgb(0.54, 0.17, 0.39), //Enemy #1: 54%, 17%, 39%
    Color::rgb(0.45, 0.23, 0.4),  // Enemy #2: 45%, 23%, 44%
    Color::rgb(0.36, 0.30, 0.49), // Enemy #3: 36%, 30%, 49%
];
