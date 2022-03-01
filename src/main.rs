use bevy::core::FixedTimestep;
use bevy::prelude::*;
use consts::*;
use rand::prelude::*;

mod consts;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Game jam 2022!".to_string(),
            width: 750.0,
            height: 500.0,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(setup_player)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(position_translation),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(spawn_next_piece),
        )
        .add_system(falling_system)
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Shape;

#[derive(Component)]
struct Falling;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_player(mut commands: Commands) {}

fn spawn_next_piece(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BOX_COLOR[rand::thread_rng().gen_range(0, BOX_COLOR.len())],
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(20.0, 20.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Shape)
        .insert(Falling)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32),
            y: ARENA_HEIGHT as f32, //always spawn at the top
        });
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
        pos.y -= 0.01;
    }
}
