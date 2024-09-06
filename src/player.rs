#![allow(unused)]
use macroquad::prelude::*;

pub struct Player {
    pos: (f32, f32),
    size: f32,
    speed: f32,
    texture: Texture2D,
}

impl Player {
    pub fn new(pos: (f32, f32), size: f32, speed: f32, texture: Texture2D) -> Self {
        Player {
            pos,
            size,
            speed,
            texture,
        }
    }
}