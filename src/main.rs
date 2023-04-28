use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;

pub const WINDOW_WIDTH: f32 = 500.0;
pub const WINDOW_HEIGHT: f32 = 500.0;

pub const X_BOTTOM: f32 = -250.0;
pub const Y_BOTTOM: f32 = -250.0;

pub const COLOR_PLAYER: Color = Color::rgb(0.9, 0.5, 0.2);
pub const COLOR_WALL: Color = Color::rgb(0.0, 0.5, 0.5);
pub const COLOR_BACK_GROUND: Color = Color::rgb(0.2, 0.2, 0.2);

pub const GRID_X_LENGTH: u8 = 8;
pub const GRID_Y_LENGTH: u8 = 8;

pub const GRID_SIZE: f32 = WINDOW_WIDTH / GRID_X_LENGTH as f32;

pub const STAGE: &str = "
########:
#      #:
#    ###:
#   P  #:
### #  #:
#      #:
#  #   #:
########:
";

fn main() {
    App::new()
        .init_resource::<Stage>()
        .insert_resource(ClearColor(COLOR_BACK_GROUND))

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
        .add_startup_system(spawn_stage)

        .run();
}

#[derive(Component)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn translation(&self)->Vec3{
        let x: f32 = self.x as f32 * GRID_SIZE + GRID_SIZE/2.0 + X_BOTTOM;
        let y: f32 = self.y as f32 * GRID_SIZE + GRID_SIZE/2.0 + Y_BOTTOM;

        return Vec3::new(x, y, 0.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Object {
    Wall,
    Player,
    Empty,
}

impl Object {
    fn color(&self)->Color{
        match self {
            Object::Player => COLOR_PLAYER,
            Object::Wall => COLOR_WALL,
            Object::Empty => panic!(),
        }
    }
}

#[derive(Resource)]
pub struct Stage {
    squares:[[Object; GRID_X_LENGTH as usize]; GRID_Y_LENGTH as usize]
}

impl Default for Stage {
    fn default() -> Self {
        let mut squares =
            [[Object::Empty; GRID_X_LENGTH as usize]; GRID_Y_LENGTH as usize];

        let mut i_y:isize = (GRID_Y_LENGTH - 1) as isize;
        let mut i_x:usize = 0;

        for char in STAGE.chars() {
            match char {
                '\n' => continue,
                ' ' => squares[i_y as usize][i_x] = Object::Empty,
                ':' => {
                    i_y -= 1;
                    i_x = 0;

                    continue;
                },
                '#' => squares[i_y as usize][i_x] = Object::Wall ,
                'P' => squares[i_y as usize][i_x] = Object::Player,
                _ =>{
                    panic!();
                },
            }

            i_x += 1;
        }

        Stage {
            squares: squares
        }
    }

}

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


