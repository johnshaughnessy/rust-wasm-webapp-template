mod ui;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello there, {}!", name));
}

#[wasm_bindgen]
pub struct Game {
    counter: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Game { counter: 0 }
    }

    pub fn tick(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}
