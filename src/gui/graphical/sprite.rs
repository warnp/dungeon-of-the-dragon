use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Layer {
    BACKGROUND,
    MOVABLES,
    UI
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
}