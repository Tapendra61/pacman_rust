use macroquad::prelude::*;
use map::*;

pub mod map;

#[macroquad::main("Hello, world!")]
async fn main() {
    let background_color = Color::new(0.45, 0.45, 0.45, 1.0);
    let map = Map::new([
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 2, 0, 2, 0, 2, 0, 1],
        [1, 0, 1, 1, 0, 1, 1, 0, 0, 1],
        [1, 0, 2, 0, 0, 2, 0, 0, 2, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 2, 0, 0, 0, 0, 1],
        [1, 0, 1, 1, 0, 1, 1, 0, 0, 1],
        [1, 0, 2, 0, 0, 2, 0, 0, 2, 1],
        [1, 0, 0, 2, 0, 2, 0, 2, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]);

    loop {
        clear_background(background_color);

        map.draw(32.0);

        next_frame().await;
    }
}
