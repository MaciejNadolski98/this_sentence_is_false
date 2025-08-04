use bevy::prelude::*;

use crate::states::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu)
            .add_systems(Update, button_interaction);
    }
}

const BACKGROUND_COLOR: Color = Color::srgb_u8(201, 241, 243);

#[derive(Component)]
struct MainMenu;

fn spawn_menu(mut commands: Commands) {
    info!("Spawning main menu");
    commands.spawn((
        MainMenu,
        Name::new("Background"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
    )).with_children(|commands| {
        commands.spawn((
            Name::new("Main Menu Text"),
            Text::new("This sentence is false"),
            TextColor(Color::BLACK),
            TextFont {
                font_size: 50.0,
                ..default()
            },
            Node {
                ..default()
            }
        ));
        commands.spawn(button(Action::Play));
        commands.spawn(button(Action::Quit));
    });
}

#[derive(Component)]
enum Action {
    Play,
    Quit,
}

fn button(action: Action) -> impl Bundle {
    let text = match action {
        Action::Play => "Play",
        Action::Quit => "Quit",
    };
    
    (
        Name::new(text),
        Button,
        action,
        Text::new(text),
        TextColor(Color::BLACK),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        Node {
            ..default()
        },
    )
}

fn button_interaction(
    mut interaction_query: Query<(&Interaction, &Action, &mut TextColor), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, action, mut text_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    Action::Play => {
                        next_state.set(GameState::InGame);
                    }
                    Action::Quit => {
                        exit.write(AppExit::Success);
                    }
                }
            },
            Interaction::Hovered => {
                text_color.0 = Color::srgb(0.5, 0.5, 0.5);
            }
            Interaction::None => {
                text_color.0 = Color::BLACK;
            }
        }
    }
}

fn despawn_menu(
    mut commands: Commands, 
    menu: Single<Entity, With<MainMenu>>
) {
    commands.entity(*menu).despawn();
}
