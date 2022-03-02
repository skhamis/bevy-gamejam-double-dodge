use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use consts::*;
use rand::prelude::*;
use std::convert::TryFrom;

mod consts;
mod scoreboard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(scoreboard::ScoreboardPlugin)
        .insert_resource(WindowDescriptor {
            title: "Bevy Game jam 2022!".to_string(),
            width: 500.0,
            height: 500.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.16, 0.17, 0.25)))
        //.add_startup_system(setup_background)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_player)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(position_translation),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(basket_movement_system)
                .with_system(falling_system)
                .with_system(shape_collision_system),
        )
        .add_system(basket_change_color_system)
        .add_system_set(
            SystemSet::new()
                // TODO: This should increase either with time or score or both??
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_next_piece),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
enum Collider {
    Solid,
    Scorable,
    Basket,
}

#[derive(Component, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Creature {
    creature_type: CreatureType,
}

#[derive(Component)]
struct Falling;

#[derive(Component)]
struct Basket {
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

fn setup_player(mut commands: Commands) {
    // basket
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -215.0, 100.0),
                scale: Vec3::new(120.0, 30.0, 100.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: BOX_COLOR[0],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Basket {
            speed: 500.0,
            item_type: CreatureType::Blue,
        })
        .insert(Collider::Basket);
}

fn spawn_next_piece(mut commands: Commands, assets: Res<AssetServer>) {
    let sprite_handle = assets.load("enemies/brown_stone.png");
    let color = rand::thread_rng().gen_range(0, BOX_COLOR.len());
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_handle.clone(),
            // sprite: Sprite {
            //     color: BOX_COLOR[color],
            //     ..Default::default()
            // },
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 100.0),
                translation: Vec3::new(0., 0., 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        // TODO: Need a better way of determining color type
        .insert(Creature {
            creature_type: CreatureType::try_from(color).unwrap_or(CreatureType::Blue),
        })
        .insert(Falling)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32),
            y: ARENA_HEIGHT as f32, //always spawn at the top
        })
        .insert(Collider::Scorable);
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.width() as f32, ARENA_WIDTH as f32),
            0.0,
        );
    }
}

fn falling_system(mut query: Query<&mut Position, With<Falling>>) {
    for mut pos in query.iter_mut() {
        pos.y -= 0.05;
    }
}

fn basket_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Basket, &mut Transform)>,
) {
    let (basket, mut transform) = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += direction * basket.speed * TIME_STEP;
    // bound the paddle within the walls
    // TODO: Make this the same as the window size
    translation.x = translation.x.min(550.0).max(-550.0);
}

fn basket_change_color_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Basket, &mut Sprite)>,
) {
    let (mut basket, mut sprite) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) {
        // We increment color type + 1
        match basket.item_type {
            CreatureType::Blue => {
                basket.item_type = CreatureType::Green;
                sprite.color = Color::rgb(0., 1., 0.);
            }
            CreatureType::Green => {
                basket.item_type = CreatureType::Brown;
                sprite.color = Color::rgb(0., 0., 1.);
            }
            CreatureType::Brown => {
                basket.item_type = CreatureType::Blue;
                sprite.color = Color::rgb(1., 0., 0.);
            }
        }
    }
}

fn shape_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<scoreboard::Scoreboard>,
    mut player_query: Query<(&mut Basket, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
) {
    let (mut ball, ball_transform) = player_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, collider, transform) in collider_query.iter() {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            Vec2::new(transform.scale.x * 20., transform.scale.y * 20.), //TODO: better solution is to make the collider struct have values for this
        );
        if let Some(collision) = collision {
            // scorable colliders should be despawned and increment the scoreboard on collision
            if let Collider::Scorable = *collider {
                scoreboard.score += 1;
                commands.entity(collider_entity).despawn();
            }
        }
    }
}
