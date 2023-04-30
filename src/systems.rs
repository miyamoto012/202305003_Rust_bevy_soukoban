use super::resources::*;
use super::components::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn draw_grid(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines: ResMut<DebugLines>
) {
    let window = window_query.get_single().unwrap();
    let half_win_width = 0.5 * window.width();
    let half_win_height = 0.5 * window.height();
    let x_space = window.width() / GRID_X_LENGTH as f32;
    let y_space = window.height() / GRID_Y_LENGTH as f32;

    let mut i = -1. * half_win_height;
    while i < half_win_height {
        lines.line(
            Vec3::new(-1. * half_win_width, i, 0.0),
            Vec3::new(half_win_width, i, 0.0),
            0.0,
        );
        i += y_space;
    }

    i = -1. * half_win_width;
    while i < half_win_width {
        lines.line(
            Vec3::new(i, -1. * half_win_height, 0.0),
            Vec3::new(i, half_win_height, 0.0),
            0.0,
        );
        i += x_space;
    }

    lines.line(
        Vec3::new(0., -1. * half_win_height, 0.0),
        Vec3::new(0., half_win_height, 0.0),
        0.0,
    );
}

pub fn spawn_stage (
    mut commands: Commands,
    stage: Res<Stage>
){
    for i_y in 0..GRID_Y_LENGTH {
        for i_x in 0..GRID_X_LENGTH {
            let positon = Position { x: i_x, y: i_y };
            let squares = stage.squares[i_y as usize][i_x as usize];

            if squares == Object::Empty {
                continue;
            }


            if squares == Object::Player {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: squares.color(),
                            ..default()
                        },
                        transform: Transform {
                            scale: Vec3::new(GRID_SIZE, GRID_SIZE, 10.0),
                            translation: positon.translation(),
                            ..default()
                        },
                        ..default()
                    },
                    positon,
                    Player{},
                ));
            } else if squares == Object::Wall {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: squares.color(),
                            ..default()
                        },
                        transform: Transform {
                            scale: Vec3::new(GRID_SIZE, GRID_SIZE, 10.0),
                            translation: positon.translation(),
                            ..default()
                        },
                        ..default()
                    },
                    positon,
                ));
            }
        }
    }
}

pub fn imput_direction (
    mut player_query: Query<(Entity, &mut Position), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    stage: Res<Stage>,
){
    let direction = if keyboard_input.pressed(KeyCode::Left){
        Vec3::new(-1.0, 0.0, 0.0)
    } else if keyboard_input.pressed(KeyCode::Right){
        Vec3::new(1.0, 0.0, 0.0)
    } else if keyboard_input.pressed(KeyCode::Up){
        Vec3::new(0.0, 1.0, 0.0)
    } else if keyboard_input.pressed(KeyCode::Down){
        Vec3::new(0.0, -1.0, 0.0)
    } else {
        Vec3::ZERO
    };

    if direction != Vec3::ZERO {
        println!("direction:{}", direction);

        if let Ok((mut player_entity, mut player_posision)) = player_query.get_single_mut() {
            println!("OK");
        }
    }


}