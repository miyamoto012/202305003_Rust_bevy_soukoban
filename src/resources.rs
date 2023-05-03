use std::usize;

use bevy::prelude::*;

pub const GRID_X_LENGTH: i8 = 8;
pub const GRID_Y_LENGTH: i8 = 8;

pub const COLOR_PLAYER: Color = Color::rgb(0.8, 0.4, 0.1);
pub const COLOR_BOX: Color = Color::rgb(0.1, 0.4, 0.8);
pub const COLOR_WALL: Color = Color::rgb(0.2, 0.2, 0.2);
pub const COLOR_BACK_GROUND: Color = Color::rgb(0.6, 0.6, 0.6);

pub const OBJECT_MOVE_TIME: f32 = 0.5;

pub const STAGE: &str = "
########:
#B     #:
#  B ###:
#   P  #:
### #  #:
#  B   #:
#  #   #:
########:
";

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Object {
    Wall,
    Player,
    Box,
    Empty,
}

impl Object {
    pub fn color(&self)->Color{
        match self {
            Object::Player => COLOR_PLAYER,
            Object::Wall => COLOR_WALL,
            Object::Box => COLOR_BOX,
            Object::Empty => panic!(),
        }
    }
}

#[derive(Resource)]
pub struct Stage {
   pub squares:[[Object; GRID_X_LENGTH as usize]; GRID_Y_LENGTH as usize]
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
                'B' => squares[i_y as usize][i_x] = Object::Box,
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

impl Stage {
    pub fn swap (&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)){
        let buff = self.squares[y1][x1];
        self.squares[y1][x1] = self.squares[y2][x2];
        self.squares[y2][x2] = buff;
    }
}

#[derive(Resource)]
pub struct ObjectMoveTimer {
    pub timer: Timer,
}

impl Default for ObjectMoveTimer {
    fn default() -> Self {
        ObjectMoveTimer {
            timer: Timer::from_seconds(OBJECT_MOVE_TIME, TimerMode::Repeating),
        }
    }
}