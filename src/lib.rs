use wasm_bindgen::prelude::*;

use retrospector::app::{run, App, AppConfig};
use retrospector::render::{clear, draw_image, Location, Renderer, SpriteBuilder};
use retrospector::update::KeyEvent;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let app = Tetris::new();
    let config = AppConfig::new(String::from("canvas"), 320.0, 640.0);
    run(app, config)
}

struct Tetris {
    elapsed_time: f64,
    mino_sprites: SpriteBuilder,
}

impl Tetris {
    fn new() -> Self {
        let bytes = include_bytes!("./assets/sprites/minos.gif");
        let mino_sprites = SpriteBuilder::new(bytes, "gif", 32.0, 32.0);
        Self {
            elapsed_time: 0.0,
            mino_sprites,
        }
    }
}

impl App for Tetris {
    fn update(&mut self, elapsed_time: f64, key_event: &KeyEvent) {
        self.elapsed_time = elapsed_time;
    }

    fn render(&self, renderer: &Renderer) {
        clear(renderer);
    }
}
