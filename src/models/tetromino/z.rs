use crate::models::{
    block::{Block, Direction},
    tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Z {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for Z {
    fn move_(&mut self, move_dir: MoveDirection) {
        let dir = Direction::from(move_dir);
        self.set_axis(self.axis().move_(dir));
    }
    fn rotate(&mut self, rotate_dir: RotateDirection) {
        let dir = self.dir().rotate(rotate_dir);
        self.set_dir(dir);
    }
    fn dry_move(&self, move_dir: MoveDirection) -> Vec<Block> {
        let dir = Direction::from(move_dir);
        Self::new(*self.dir(), self.axis().move_(dir)).blocks()
    }
    fn dry_rotate(&self, rotate_dir: RotateDirection) -> Vec<Block> {
        let dir = self.dir().rotate(rotate_dir);
        Self::new(dir, *self.axis()).blocks()
    }
    fn blocks(&self) -> Vec<Block> {
        match self.dir() {
            TetrominoDirection::North => vec![
                self.axis().move_(Direction::Up).move_(Direction::Left),
                self.axis().move_(Direction::Up),
                *self.axis(),
                self.axis().move_(Direction::Right),
            ],
            TetrominoDirection::East => vec![
                self.axis().move_(Direction::Right).move_(Direction::Up),
                self.axis().move_(Direction::Right),
                *self.axis(),
                self.axis().move_(Direction::Down),
            ],
            TetrominoDirection::South => vec![
                self.axis().move_(Direction::Left),
                *self.axis(),
                self.axis().move_(Direction::Down),
                self.axis().move_(Direction::Down).move_(Direction::Right),
            ],
            TetrominoDirection::West => vec![
                self.axis().move_(Direction::Left).move_(Direction::Down),
                self.axis().move_(Direction::Left),
                *self.axis(),
                self.axis().move_(Direction::Up),
            ],
        }
    }
}

impl Z {
    pub fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
}

impl Z {
    fn dir(&self) -> &TetrominoDirection {
        &self.dir
    }
    fn axis(&self) -> &Block {
        &self.axis
    }
    fn set_dir(&mut self, dir: TetrominoDirection) {
        self.dir = dir;
    }
    fn set_axis(&mut self, axis: Block) {
        self.axis = axis;
    }
}
