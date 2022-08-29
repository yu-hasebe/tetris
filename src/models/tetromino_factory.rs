use std::collections::VecDeque;

use crate::models::{
    block::{Block, Color},
    tetromino::{i::I, j::J, l::L, o::O, s::S, t::T, z::Z, Tetromino, TetrominoDirection},
};

pub struct TetrominoFactory {
    seven_bag: VecDeque<Box<dyn Tetromino>>,
    rand: usize,
}

impl TetrominoFactory {
    pub fn new() -> Self {
        Self {
            seven_bag: VecDeque::new(),
            rand: 0,
        }
    }

    pub fn pop(&mut self) -> Box<dyn Tetromino> {
        if self.seven_bag.len() < 7 {
            let mut new_seven_bag = self.new_seven_bag();
            self.seven_bag.append(&mut new_seven_bag);
        }
        self.seven_bag.pop_front().unwrap()
    }

    pub fn next_tetromino(&self, index: usize) -> Vec<Block> {
        todo!()
    }

    fn new_seven_bag(&mut self) -> VecDeque<Box<dyn Tetromino>> {
        self.fisher_yates_shuffle(&mut Self::build_seven_tetrominos())
    }

    fn fisher_yates_shuffle(
        &mut self,
        to_shuffle: &mut VecDeque<Box<dyn Tetromino>>,
    ) -> VecDeque<Box<dyn Tetromino>> {
        let mut ret = VecDeque::new();
        let len = to_shuffle.len();
        for i in (1..=len).rev() {
            let rand = self.linear_congruential_generate();
            let idx = rand % i;
            let removed = to_shuffle.remove(idx).unwrap();
            ret.push_back(removed);
        }
        ret
    }

    fn linear_congruential_generate(&mut self) -> usize {
        let next = (13 * self.rand + 5) % 24;
        self.rand = next;
        next
    }
}

impl TetrominoFactory {
    fn build_seven_tetrominos() -> VecDeque<Box<dyn Tetromino>> {
        let mut ret: VecDeque<Box<dyn Tetromino>> = VecDeque::new();
        ret.push_back(Box::new(Self::build_default_i()));
        ret.push_back(Box::new(Self::build_default_j()));
        ret.push_back(Box::new(Self::build_default_l()));
        ret.push_back(Box::new(Self::build_default_s()));
        ret.push_back(Box::new(Self::build_default_z()));
        ret.push_back(Box::new(Self::build_default_t()));
        ret.push_back(Box::new(Self::build_default_o()));
        ret
    }

    fn build_default_i() -> I {
        I::new(TetrominoDirection::North, Block::new(Color::Cyan, 4, 20))
    }

    fn build_default_j() -> J {
        J::new(TetrominoDirection::North, Block::new(Color::Blue, 4, 20))
    }

    fn build_default_l() -> L {
        L::new(TetrominoDirection::North, Block::new(Color::Orange, 4, 20))
    }

    fn build_default_s() -> S {
        S::new(TetrominoDirection::North, Block::new(Color::Green, 4, 20))
    }

    fn build_default_z() -> Z {
        Z::new(TetrominoDirection::North, Block::new(Color::Red, 4, 20))
    }

    fn build_default_t() -> T {
        T::new(TetrominoDirection::North, Block::new(Color::Purple, 4, 20))
    }

    fn build_default_o() -> O {
        O::new(TetrominoDirection::North, Block::new(Color::Yellow, 4, 20))
    }
}
