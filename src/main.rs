use bevy::{input::common_conditions::input_just_pressed, prelude::*};

mod menu;
mod states;
mod game;


use menu::MenuPlugin;
use states::GameState;
use game::GamePlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          fit_canvas_to_parent: true,
          ..default()
        }),
        ..default()
    }))
    .init_state::<GameState>()
    .add_plugins((MenuPlugin, GamePlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, app_exit.run_if(input_just_pressed(KeyCode::Escape)))
    .run();
}

fn app_exit(mut exit: EventWriter<AppExit>) {
  exit.write(AppExit::Success);
}

fn spawn_camera(mut commands: Commands) {
    info!("Spawning camera");
    commands.spawn((
        Name::new("Camera"), 
        Camera2d
    ));
  }
