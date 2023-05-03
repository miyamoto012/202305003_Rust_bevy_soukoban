pub mod components;
pub mod resources;
pub mod states;
mod systems;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use systems::*;
use resources::*;
use components::*;
use states::*;

fn main() {
    App::new()
        .init_resource::<Stage>()
        .init_resource::<ObjectMoveTimer>()
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
        .add_startup_system(spawn_stage)

        .add_system(bevy::window::close_on_esc)
        .add_plugin(DebugLinesPlugin::default())
        .add_system(draw_grid)

        .add_system(
            input_direction
            .run_if(in_state(PlayerState::Stop))
        )
        .add_system(
            reflect_position
            .in_schedule(OnExit(PlayerState::Stop))
        )

        .add_system(
            object_movement
            .run_if(in_state(PlayerState::Move))
        )
        .add_system(
            debug_print_stage
            .in_schedule(OnExit(PlayerState::Move))
        )

        .run();
}
