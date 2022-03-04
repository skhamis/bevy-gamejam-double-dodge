use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use consts::*;
use rand::prelude::*;
use std::convert::TryFrom;

mod consts;
mod menu;
mod scoreboard;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.16, 0.17, 0.25)))
        .insert_resource(WindowDescriptor {
            title: "Dodge the rock creatures!".to_string(),
            width: 1000.0,
            height: 600.0,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Menu)
        //.add_state_to_stage(GameState::Menu)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(scoreboard::ScoreboardPlugin)
        // .add_state_to_stage(CoreStage::Update, GameState::Menu)
        //.add_startup_system(setup_background)
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                //SystemSet::new()
                .with_system(setup_camera)
                .with_system(setup_walls)
                .with_system(setup_player1)
                .with_system(setup_player2),
        )
        // //.add_startup_system(setup_player1)
        // //.add_startup_system(setup_player2)
        // .add_system_set_to_stage(
        //     CoreStage::PostUpdate,
        //     SystemSet::on_update(GameState::InGame).with_system(position_translation),
        // )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                //.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player1_movement_system)
                .with_system(player2_movement_system)
                .with_system(moving_system)
                .with_system(player1_collision_system)
                .with_system(player2_collision_system),
        )
        // //.add_system(player_change_color_system)
        .add_system_set(
            //SystemSet::on_update(GameState::InGame)
            SystemSet::new()
                // TODO: This should increase either with time or score or both??
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(spawn_next_piece),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(timer_score_system),
        )
        .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(teardown))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
enum Collider {
    Solid,
    Scorable,
    Player,
    Wall,
}

#[derive(Component, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Creature;
//creature_type: CreatureType,

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

// impl TryFrom<usize> for Direction {
//     type Error = ();

//     fn try_from(v: usize) -> Result<Self, Self::Error> {
//         match v {
//             x if x == Direction::Up as usize => Ok(Direction::Down),
//             x if x == Direction::Down as usize => Ok(Direction::Left),
//             x if x == Direction::Left as usize => Ok(Direction::Right),
//             x if x == Direction::Right as usize => Ok(Direction::Up),
//             _ => Err(()),
//         }
//     }
// }

// impl Iterator for Direction {
//     // we will be counting with usize
//     type Item = Direction;

//     // next() is the only required method
//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             &mut Direction::Up => Some(Direction::Down),
//             &mut Direction::Down => Some(Direction::Left),
//             &mut Direction::Left => Some(Direction::Right),
//             &mut Direction::Right => Some(Direction::Up),
//         }
//     }
// }

#[derive(Component)]
struct Moving {
    direction: Direction,
}

#[derive(Component)]
struct Player1 {
    speed: f32,
    item_type: CreatureType,
}

#[derive(Component)]
struct Player2 {
    speed: f32,
    item_type: CreatureType,
}

// Type of colors things can be
#[derive(Debug, Eq, PartialEq)]
//#[repr(usize)]
enum CreatureType {
    Blue = 0,
    Green = 1,
    Brown = 2,
}

impl TryFrom<usize> for CreatureType {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == CreatureType::Blue as usize => Ok(CreatureType::Blue),
            x if x == CreatureType::Green as usize => Ok(CreatureType::Green),
            x if x == CreatureType::Brown as usize => Ok(CreatureType::Brown),
            _ => Err(()),
        }
    }
}

// fn setup_background(mut commands: Commands, assets: Res<AssetServer>) {
//     let background_image = assets.load("bg/full-bg.png");

//     commands.spawn_bundle(SpriteBundle {
//         texture: background_image.clone(),
//         transform: Transform::from_scale(Vec3::new(1., 1., 0.0)),
//         ..Default::default()
//     });
//}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_walls(mut commands: Commands) {
    // basket
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 100.0),
                scale: Vec3::new(3.0, 800.0, 100.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR[1],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Wall);
}

fn setup_player1(mut commands: Commands) {
    // basket
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-215.0, -215.0, 100.0),
                scale: Vec3::new(50.0, 50.0, 100.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR[0],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player1 {
            speed: 500.0,
            item_type: CreatureType::Blue,
        })
        .insert(Collider::Player);
}

fn setup_player2(mut commands: Commands) {
    // basket
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(215.0, -215.0, 100.0),
                scale: Vec3::new(50.0, 50.0, 100.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR[2],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player2 {
            speed: 500.0,
            item_type: CreatureType::Blue,
        })
        .insert(Collider::Player);
}

fn spawn_next_piece(
    windows: Res<Windows>,
    state: ResMut<State<GameState>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    if *state.current() != GameState::InGame {
        return;
    }
    let mut rng = rand::thread_rng();
    //let rand_type = rand::thread_rng().gen_range(0, CREATURE_TYPE.len());
    let creature = CREATURE_TYPE.iter().choose(&mut rng).unwrap();
    let sprite_handle = assets.load(*creature);

    // We want to pick a random type of direction
    //let mut rng = rand::thread_rng();
    let flip: f32 = rng.gen_range(-1.0, 1.0);
    //let mut rng = rand::thread_rng();
    let possible_directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let direction = possible_directions.choose(&mut rng).unwrap();

    let mut x_pos: f32 = 0.0;
    let mut y_pos: f32 = 0.0;
    let window = windows.get_primary().unwrap();
    // println!("{:?}", window.width());
    // println!("{:?}", window.height());
    match &direction {
        Direction::Up => {
            // Spawn at the bottom, along x axis
            x_pos = flip * (window.width() / 2.0) as f32;
            y_pos = -1.0 * window.height() as f32;
        }
        Direction::Down => {
            x_pos = flip * (window.width() / 2.0) as f32;
            y_pos = (window.height() / 2.0) as f32;
        }
        Direction::Left => {
            x_pos = (window.height() / 2.0) as f32;
            y_pos = flip * (window.height() / 2.0) as f32;
        }
        Direction::Right => {
            x_pos = -1.0 * (window.height() / 2.0) as f32;
            y_pos = flip * (window.width() / 2.0) as f32
        }
    }

    // println!("x_pos: {:?}", x_pos);
    // println!("y_pos: {:?}", y_pos);
    // println!("direction: {:?}", direction);

    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_handle.clone(),
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 100.0),
                translation: Vec3::new(x_pos, y_pos, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        // TODO: Need a better way of determining color type
        // .insert(Creature {
        //     creature_type: CreatureType::try_from(rand_type).unwrap_or(CreatureType::Blue),
        // })
        .insert(Creature)
        .insert(Moving {
            direction: *direction,
        })
        .insert(Position { x: x_pos, y: y_pos })
        .insert(Collider::Scorable);
}

// fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
//     fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
//         let tile_size = bound_window / bound_game;
//         pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
//     }
//     let window = windows.get_primary().unwrap();
//     for (pos, mut transform) in q.iter_mut() {
//         transform.translation = Vec3::new(
//             convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
//             convert(pos.y as f32, window.width() as f32, ARENA_WIDTH as f32),
//             0.0,
//         );
//     }
// }

fn moving_system(mut query: Query<(&mut Transform, &Moving)>) {
    for (mut pos, moving) in query.iter_mut() {
        //println!("Moving in: {:?}", moving.direction);
        match moving.direction {
            // Direction::Up => pos.y += 1.05,
            // Direction::Down => pos.y -= 1.05,
            // Direction::Left => pos.x -= 1.05,
            // Direction::Right => pos.x += 1.05,
            Direction::Up => pos.translation.y += 1.05,
            Direction::Down => pos.translation.y -= 1.05,
            Direction::Left => pos.translation.x -= 1.05,
            Direction::Right => pos.translation.x += 1.05,
        }
    }
}

fn player1_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player1, &mut Transform)>,
) {
    let (basket, mut transform) = query.single_mut();
    let mut direction = 0.0;
    let translation = &mut transform.translation;
    if keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
        // move the paddle horizontally
        translation.x += direction * basket.speed * TIME_STEP;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += 1.0;
        // move the paddle horizontally
        translation.x += direction * basket.speed * TIME_STEP;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += 1.0;
        // move the paddle vertically
        translation.y += direction * basket.speed * TIME_STEP;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction -= 1.0;
        // move the paddle vertically
        translation.y += direction * basket.speed * TIME_STEP;
    }

    // bound the paddle within the walls
    // TODO: Make this the same as the window size
    translation.x = translation.x.min(550.0).max(-550.0);
    translation.y = translation.y.min(350.0).max(-350.0);
}

fn player2_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player2, &mut Transform)>,
) {
    let (basket, mut transform) = query.single_mut();
    let mut direction = 0.0;
    let translation = &mut transform.translation;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
        // move the paddle horizontally
        translation.x += direction * basket.speed * TIME_STEP;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
        // move the paddle horizontally
        translation.x += direction * basket.speed * TIME_STEP;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
        // move the paddle vertically
        translation.y += direction * basket.speed * TIME_STEP;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
        // move the paddle vertically
        translation.y += direction * basket.speed * TIME_STEP;
    }

    // bound the paddle within the walls
    // TODO: Make this the same as the window size
    translation.x = translation.x.min(550.0).max(-550.0);
    translation.y = translation.y.min(350.0).max(-350.0);
}

// fn player_change_color_system(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&mut Basket, &mut Sprite)>,
// ) {
//     let (mut basket, mut sprite) = query.single_mut();

//     if keyboard_input.just_pressed(KeyCode::Space) {
//         // We increment color type + 1
//         match basket.item_type {
//             CreatureType::Blue => {
//                 basket.item_type = CreatureType::Green;
//                 sprite.color = BOX_COLOR[1];
//             }
//             CreatureType::Green => {
//                 basket.item_type = CreatureType::Brown;
//                 sprite.color = BOX_COLOR[2];
//             }
//             CreatureType::Brown => {
//                 basket.item_type = CreatureType::Blue;
//                 sprite.color = BOX_COLOR[0];
//             }
//         }
//     }
// }

fn player1_collision_system(
    // mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    // mut scoreboard: ResMut<scoreboard::Scoreboard>,
    mut player_query: Query<(&mut Player1, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
) {
    let (mut basket, basket_transform) = player_query.single_mut();
    let basket_size = basket_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, collider, transform /*creature*/) in collider_query.iter() {
        let collision = collide(
            basket_transform.translation,
            basket_size,
            transform.translation,
            Vec2::new(transform.scale.x * 20., transform.scale.y * 20.), //TODO: better solution is to make the collider struct have values for this
        );
        if let Some(_collision) = collision {
            if let Collider::Scorable = *collider {
                // Only score if we catch the matching type
                //if basket.item_type == creature.creature_type {
                // scorable colliders should be despawned and increment the scoreboard on collision
                //scoreboard.score += 1;
                //commands.entity(collider_entity).despawn();
                //}
                match game_state.current() {
                    // End the game if we hit something
                    GameState::InGame => {
                        game_state.set(GameState::Menu).unwrap();
                    }
                    _ => {}
                }
            }
            // Keeeping this separate for now incase we want to do something different with walls
            if let Collider::Wall = *collider {
                match game_state.current() {
                    // End the game if we hit something
                    GameState::InGame => {
                        game_state.set(GameState::Menu).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn player2_collision_system(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut scoreboard: ResMut<scoreboard::Scoreboard>,
    mut player_query: Query<(&mut Player2, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
) {
    let (mut basket, basket_transform) = player_query.single_mut();
    let basket_size = basket_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, collider, transform) in collider_query.iter() {
        let collision = collide(
            basket_transform.translation,
            basket_size,
            transform.translation,
            Vec2::new(transform.scale.x * 20., transform.scale.y * 20.), //TODO: better solution is to make the collider struct have values for this
        );
        if let Some(_collision) = collision {
            if let Collider::Scorable = *collider {
                // Only score if we catch the matching type
                //if basket.item_type == creature.creature_type {
                // scorable colliders should be despawned and increment the scoreboard on collision
                //scoreboard.score += 1;
                //commands.entity(collider_entity).despawn();
                //}
                match game_state.current() {
                    // End the game if we hit something
                    GameState::InGame => {
                        game_state.set(GameState::Menu).unwrap();
                    }
                    _ => {}
                }
            }
            // Keeeping this separate for now incase we want to do something different with walls
            if let Collider::Wall = *collider {
                match game_state.current() {
                    // End the game if we hit something
                    GameState::InGame => {
                        game_state.set(GameState::Menu).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn timer_score_system(mut _commands: Commands, mut scoreboard: ResMut<scoreboard::Scoreboard>) {
    scoreboard.score += 1;
}

// remove all entities that are not a camera
fn teardown(
    mut commands: Commands,
    entities: Query<Entity, Without<Camera>>,
    mut scoreboard: ResMut<scoreboard::Scoreboard>,
) {
    // Reset the scoreboard
    scoreboard.score = 0;

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
