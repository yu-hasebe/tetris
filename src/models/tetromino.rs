pub mod i;
pub mod j;
pub mod l;
pub mod o;
pub mod s;
pub mod t;
pub mod z;

use crate::models::block::{Block, Direction};

pub trait Tetromino {
    fn move_(&mut self, move_dir: MoveDirection);
    fn rotate(&mut self, rotate_dir: RotateDirection);
    fn dry_move(&self, move_dir: MoveDirection) -> Vec<Block>;
    fn dry_rotate(&self, rotate_dir: RotateDirection) -> Vec<Block>;
    fn blocks(&self) -> Vec<Block>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TetrominoDirection {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
pub enum RotateDirection {
    Left,
    Right,
}

impl TetrominoDirection {
    fn rotate(&self, rotate_dir: RotateDirection) -> Self {
        match rotate_dir {
            RotateDirection::Left => match self {
                TetrominoDirection::North => TetrominoDirection::West,
                TetrominoDirection::West => TetrominoDirection::South,
                TetrominoDirection::South => TetrominoDirection::East,
                TetrominoDirection::East => TetrominoDirection::North,
            },
            RotateDirection::Right => match self {
                TetrominoDirection::North => TetrominoDirection::East,
                TetrominoDirection::West => TetrominoDirection::North,
                TetrominoDirection::South => TetrominoDirection::West,
                TetrominoDirection::East => TetrominoDirection::South,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MoveDirection {
    Left,
    Right,
    Down,
}

impl From<MoveDirection> for Direction {
    fn from(move_dir: MoveDirection) -> Self {
        match move_dir {
            MoveDirection::Left => Direction::Left,
            MoveDirection::Right => Direction::Right,
            MoveDirection::Down => Direction::Down,
        }
    }
}
