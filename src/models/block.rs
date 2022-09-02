use anyhow::Result;
use derive_new::new;

use retrospector::render::{draw_image, Location, Renderer, SpriteStore};

#[derive(Clone, Copy, Debug, Eq, new, PartialEq)]
pub struct Block {
    color: Color,
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Cyan,
    Blue,
    Orange,
    Green,
    Red,
    Purple,
    Yellow,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Block {
    pub fn move_(&self, dir: Direction) -> Self {
        let color = self.color;
        let (x, y) = match dir {
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y - 1),
            Direction::Up => (self.x, self.y + 1),
        };
        Self { color, x, y }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn x(&self) -> &i32 {
        &self.x
    }

    pub fn y(&self) -> &i32 {
        &self.y
    }

    pub fn render(&self, renderer: &Renderer, tetromino_sprites: &SpriteStore) -> Result<()> {
        let col = match self.color() {
            Color::Cyan => 1,
            Color::Blue => 2,
            Color::Orange => 3,
            Color::Green => 4,
            Color::Red => 5,
            Color::Purple => 6,
            Color::Yellow => 7,
        };
        if let Ok(sprite) = tetromino_sprites.sprite(col, 0) {
            let location = Location::new(*self.x() as f64 * 32.0, (19.0 - *self.y() as f64) * 32.0);
            draw_image(renderer, &sprite, location)?;
        }

        Ok(())
    }
}
