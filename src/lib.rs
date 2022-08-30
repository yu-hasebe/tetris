mod models;

use std::result::Result;

use wasm_bindgen::prelude::*;

use retrospector::app::{run, App, AppConfig};
use retrospector::render::{clear, Renderer, SpriteBuilder};
use retrospector::update::KeyEvent;

use models::field::Field;
use models::tetromino::{MoveDirection, RotateDirection, Tetromino};
use models::tetromino_factory::TetrominoFactory;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let app = Tetris::new();
    let config = AppConfig::new(String::from("canvas"), 320.0, 640.0);
    run(app, config)
}

struct Tetris {
    field: Field,
    tetromino_factory: TetrominoFactory,
    tetromino: Box<dyn Tetromino>,
    tetromino_sprites: SpriteBuilder,
    updated_at: f64,
}

impl Tetris {
    fn new() -> Self {
        let field = Field::new(vec![vec![None; 10]; 24]);
        let mut tetromino_factory = TetrominoFactory::new();
        let tetromino = tetromino_factory.pop();

        let bytes = include_bytes!("./assets/sprites/minos.gif");
        let tetromino_sprites = SpriteBuilder::new(bytes, "gif", 32.0, 32.0);
        let updated_at = 0.0;
        Self {
            field,
            tetromino_factory,
            tetromino,
            tetromino_sprites,
            updated_at,
        }
    }
}

impl App for Tetris {
    fn update(&mut self, elapsed_time: f64, key_event: &KeyEvent) {
        if elapsed_time - self.updated_at < 100.0 {
            return;
        }

        if elapsed_time - self.updated_at > 300.0 || key_event.is_arrow_down_down() {
            let blocks = self.tetromino.dry_move(MoveDirection::Down);
            if self.field.is_vacant(&blocks) {
                self.tetromino.move_(MoveDirection::Down);
                self.updated_at = elapsed_time;
                return;
            }

            if Field::can_fix(&blocks) {
                let blocks = self.tetromino.blocks();
                self.field.fix_blocks(blocks);
                self.field.clear_blocks();
            } else {
                // game over
            }

            self.tetromino = self.tetromino_factory.pop();
            if !self.field.is_vacant(&self.tetromino.blocks()) {
                // game over
            }
        }

        if key_event.is_arrow_left_down() {
            let blocks = self.tetromino.dry_move(MoveDirection::Left);
            if self.field.is_vacant(&blocks) {
                self.tetromino.move_(MoveDirection::Left);
                self.updated_at = elapsed_time;
            }
        } else if key_event.is_arrow_right_down() {
            let blocks = self.tetromino.dry_move(MoveDirection::Right);
            if self.field.is_vacant(&blocks) {
                self.tetromino.move_(MoveDirection::Right);
                self.updated_at = elapsed_time;
            }
        } else if key_event.is_key_z_down() {
            let blocks = self.tetromino.dry_rotate(RotateDirection::Left);
            if self.field.is_vacant(&blocks) {
                self.tetromino.rotate(RotateDirection::Left);
                self.updated_at = elapsed_time;
            }
        } else if key_event.is_key_x_down() {
            let blocks = self.tetromino.dry_rotate(RotateDirection::Right);
            if self.field.is_vacant(&blocks) {
                self.tetromino.rotate(RotateDirection::Right);
                self.updated_at = elapsed_time;
            }
        }
    }

    fn render(&self, renderer: &Renderer) {
        clear(renderer);
        for block in self.tetromino.blocks() {
            block.render(renderer, &self.tetromino_sprites);
        }
        for block in self.field.blocks() {
            block.render(renderer, &self.tetromino_sprites);
        }
    }
}
