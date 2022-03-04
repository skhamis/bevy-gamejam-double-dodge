use crate::GameState;
use bevy::prelude::*;

pub struct ScoreboardPlugin;

pub(crate) struct Scoreboard {
    pub(crate) score: usize,
}

#[derive(Component)]
struct ScoreText;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(Scoreboard { score: 0 })
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(scoreboard_system))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(teardown));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.5, 0.5),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText);
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<(&mut Text, &ScoreText)>) {
    let (mut text, _score_text) = query.single_mut();
    text.sections[1].value = format!("{}", scoreboard.score);
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    println!("Scoreboard teardown called");
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
