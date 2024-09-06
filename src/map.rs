use macroquad::prelude::*;

pub struct Map {
    layout: [[i32; 10]; 10],
}

impl Map {
    pub fn new(layout: [[i32; 10]; 10]) -> Map {
        Map { layout }
    }

    pub fn draw(&self, tile_size: f32) {
        for (y, row) in self.layout.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let position = Vec2::new(x as f32 * tile_size, y as f32 * tile_size);

                match tile {
                    1 => {
                        draw_rectangle(position.x, position.y, tile_size, tile_size, BLUE);
                    }

                    2 => {
                        draw_circle(
                            position.x + tile_size / 2.0,
                            position.y + tile_size / 2.0,
                            5.0,
                            WHITE,
                        );
                    }

                    _ => {
                        draw_rectangle(position.x, position.y, tile_size, tile_size, BLACK);
                    }
                }
            }
        }
    }
}
