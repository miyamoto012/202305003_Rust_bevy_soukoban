use super::resources::*;
use super::components::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;

use super::states::*;

pub const OBJECT_MOVE_SPEED: f32 = GRID_SIZE / OBJECT_MOVE_TIME;

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
                ));
            } else if squares == Object::Box {
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
                    Box {},
                ));
            }
        }
    }
}

pub fn input_direction (
    mut commands: Commands,
    mut player_query: Query<(Entity, &Position), With<Player>>,
    mut box_query: Query<(Entity, &Position), (With<Box>, Without<Player>)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut stage: ResMut<Stage>,
){

    if let Ok((player_entity, player_posision)) = player_query.get_single_mut() {

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

        if direction == Vec3::ZERO {
            return;
        }

        // 移動に伴い座標の値が変化する座標を保存
        let mut swap_position_list: Vec<Position> = Vec::new();

        swap_position_list.push(Position { x: player_posision.x, y: player_posision.y });

        // 移動先に障害物にないか調査
        let mut i_x = (player_posision.x as isize + direction.x as isize) as usize;
        let mut i_y = (player_posision.y as isize + direction.y as isize) as usize;

        while stage.squares[i_y][i_x] != Object::Empty && stage.squares[i_y][i_x] != Object::Wall {
            swap_position_list.push(Position { x: i_x as i8, y: i_y as i8});
            
            i_x = (i_x as isize + direction.x as isize) as usize;
            i_y = (i_y as isize + direction.y as isize) as usize;
        }
        swap_position_list.push(Position { x: i_x as i8, y: i_y as i8});

        // 移動先に壁があった場合は移動をしない
        if stage.squares[i_y][i_x] == Object::Wall {
            return;
        }

        // 移動に併せて配列stageの中身を変更
        for index in (1..swap_position_list.len()).rev() {
            stage.swap(
                (swap_position_list[index].x as usize, swap_position_list[index].y as usize), 
                (swap_position_list[index-1].x as usize, swap_position_list[index-1].y as usize)
            );
        }

        for (box_entity, box_position) in box_query.iter_mut() {
            if swap_position_list.iter().any(|position| 
                position.x == box_position.x && position.y == box_position.y)
            {
                commands.entity(box_entity).insert(MoveDirection{
                    vec3: direction
                });
            }
        }

        commands.entity(player_entity)
            .insert(MoveDirection{
                vec3: direction
            });

        commands.insert_resource(NextState(Some(PlayerState::Move)));

    }
}

pub fn reflect_position (
    mut object_query: Query<(&mut Position, &MoveDirection)>,
) {
    for (mut position, move_direction) in object_query.iter_mut() {
        position.x = (position.x as isize + move_direction.vec3.x as isize) as i8;
        position.y = (position.y as isize + move_direction.vec3.y as isize) as i8;
    }
}

pub fn object_movement (
    mut commands: Commands,
    mut object_move_timer: ResMut<ObjectMoveTimer>,
    mut object_query: Query<(Entity, &Position, &mut Transform, &MoveDirection)>,
    time: Res<Time>,
) {
    for (entity, posision, mut transform, move_direction) in object_query.iter_mut(){
        transform.translation += move_direction.vec3 * OBJECT_MOVE_SPEED * time.delta_seconds();

        if object_move_timer.timer.finished() {
            println!("stop");

           transform.translation = posision.translation();

            commands.entity(entity)
                .remove::<MoveDirection>();

            commands.insert_resource(NextState(Some(PlayerState::Stop)));
        }
    }
    object_move_timer.timer.tick(time.delta());
}

pub fn debug_print_stage (
    stage: Res<Stage>,
) {

    for i_y in (0..GRID_Y_LENGTH).rev() {
        for i_x in 0..GRID_X_LENGTH {
            let c = match stage.squares[i_y as usize][i_x as usize] {
                Object::Player => 'P', 
                Object::Box => 'B', 
                Object::Wall => '#', 
                Object::Empty => ' ', 
            };
            print!("{}", c);
        }
        print!("\n");
    } 

}