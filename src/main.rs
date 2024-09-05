use macroquad::prelude::*;

#[macroquad::main("Hello, world!")]
async fn main() {
    let background_color = Color::new(0.45, 0.45, 0.45, 1.0);
    clear_background(background_color);
    loop {
        
        next_frame().await;
    }
}
