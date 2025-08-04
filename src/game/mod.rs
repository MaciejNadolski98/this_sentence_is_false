use bevy::prelude::*;

use crate::{game::{levels::{CurrentLevel, LevelPlugin}, sentence::{evaluate_sentences, SentencePlugin}}, states::GameState};

mod sentence;
mod levels;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((SentencePlugin, LevelPlugin))
            .add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::InGame), (spawn_game, set_level).chain())
            .add_systems(Update, (checkbox_interaction, evaluate_interaction))
            .add_systems(OnExit(GameState::InGame), despawn_game);
    }
}

const BACKGROUND_COLOR: Color = Color::srgb_u8(201, 241, 243);

#[allow(dead_code)]
#[derive(Resource)]
struct CheckboxAssets(Handle<Image>, Handle<Image>);

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(CheckboxAssets(
        asset_server.load("checkbox_true.png"),
        asset_server.load("checkbox_false.png"),
    ));
}

#[derive(Component)]
struct Game;

fn spawn_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Game,
        Name::new("Background"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
    )).with_children(|commands| {
        commands
            .spawn(notebook(asset_server.clone()));
    });
}

#[derive(Component)]
struct Notebook;

#[derive(Component)]
struct TextBox;

#[derive(Component)]
struct CheckboxContainer;

#[derive(Component)]
struct Evaluate;

fn notebook(asset_server: AssetServer) -> impl Bundle {
    (
        Name::new("Notebook"),
        Notebook,
        ImageNode::new(asset_server.load("notebook.png")),
        Node {
            width: Val::Px(1320.0),
            height: Val::Px(753.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        related!(Children[
            (
                Name::new("Text box"),
                TextBox,
                Node {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    left: Val::Px(273.0),
                    right: Val::Px(128.0),
                    top: Val::Px(219.0),
                    bottom: Val::Px(6.0),
                    ..default()
                },
            ),
            (
                Name::new("Check boxes"),
                CheckboxContainer,
                Node {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    left: Val::Px(229.0),
                    right: Val::Px(1060.0),
                    top: Val::Px(219.0),
                    bottom: Val::Px(6.0),
                    ..default()
                },
            ),
            (
                Name::new("Evaluate"),
                Button,
                Evaluate,
                ImageNode::new(asset_server.load("play.png")),
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(120.0),
                    right: Val::Px(200.0),
                    ..default()
                },
            ),
        ]),
    )
}

fn set_level(
    mut level: ResMut<CurrentLevel>,
) {
    level.0 = 1;
}

#[derive(Component)]
struct Checkbox(bool);

fn checkbox(asset_server: AssetServer) -> impl Bundle {
    (
        Name::new("Checkbox"),
        Button,
        Checkbox(true),
        ImageNode::new(asset_server.load("checkbox_true.png")),
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(8.0), Val::Px(8.0)),
            ..default()
        },
    )
}

fn checkbox_interaction(
    interaction_query: Query<(&Interaction, &mut Checkbox, &mut ImageNode), (Changed<Interaction>, With<Button>)>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut checkbox, mut image_node) in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                checkbox.0 = !checkbox.0;
                *image_node = ImageNode::new(asset_server.load(if checkbox.0 {
                    "checkbox_true.png"
                } else {
                    "checkbox_false.png"
                }));
            }
            _ => {}
        }
    }
}

fn evaluate_interaction(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<Evaluate>)>,
) {
    for &interaction in interaction_query.iter() {
        if interaction == Interaction::Pressed {
            commands.run_system_cached(evaluate_sentences);
        }
    }
}

fn despawn_game(
    mut commands: Commands, 
    menu: Single<Entity, With<Game>>
) {
    commands.entity(*menu).despawn();
}
