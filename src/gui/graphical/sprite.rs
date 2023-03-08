use std::collections::BTreeMap;
use ggez::glam::Vec2;
use ggez::graphics::{DrawParam, Image};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Layer {
    BACKGROUND,
    MOVABLES,
    UI,
    PARTICLE
}

pub trait ObjectToSprite {
    fn get_world_origin(&self) -> Vec<Sprite>;
}

#[derive(Clone,Serialize, Deserialize, Debug)]
pub struct Sprite {
    pub texture_id: u8,
    pub pos_x: i32,
    pub pos_y: i32,
    pub layer: Layer
}

impl Sprite {
    pub fn new(texture_id: u8, pos_x: i32, pos_y: i32, layer: Layer) -> Self {
        Sprite{
            texture_id,
            pos_x,
            pos_y,
            layer
        }
    }

    pub fn create_drawable(&self, sprite_size: f32, sprite_textures: &BTreeMap<u8, Image>) -> (Image, DrawParam){
        let param = DrawParam::new().dest(Vec2::new(self.pos_x as f32* sprite_size, self.pos_y as f32 * sprite_size));
        (sprite_textures.get(&self.texture_id).unwrap().clone(), param)
    }
}