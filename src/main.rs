pub mod components;
mod systems;
pub mod resources;

use bevy::prelude::*;
use systems::*;
use resources::*;
use components::*;
use bevy_prototype_debug_lines::*;

fn main() {
    App::new()
        .init_resource::<Stage>()
        .insert_resource(ClearColor(COLOR_BACK_GROUND))
        .add_state::<PlayerState>()

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "soukoban!".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera)

        .add_system(bevy::window::close_on_esc)
        .add_plugin(DebugLinesPlugin::default())
        .add_system(draw_grid)
        .add_system(imput_direction)
        .add_startup_system(spawn_stage)

        .run();
}
