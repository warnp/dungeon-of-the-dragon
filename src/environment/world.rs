use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::graphical::sprite::{Layer, ObjectToSprite, Sprite};
use crate::pawn::pawn::Pawn;

#[derive(Debug)]
pub struct World{
    pub name: String,
    pub places: Vec<Place>,
    pub day: u32,
}

impl World {
    pub fn add_day(&mut self){
        self.day += 1;
    }
}

#[derive(Debug)]
pub struct Weather{
    pub name: String,
    pub temperature: i8,
    pub humidity: u8,
    pub visibility: u8,
    pub wind: u8
}

#[derive(Debug)]
pub struct Place {
    pub id: u8,
    pub name: String,
    pub weather: Rc<Weather>,
    pub time: String,
    pub light: u8,
    pub adjacent_places: Vec<u8>,
    pub pawns: Vec<Rc<RefCell<Pawn>>>,
    pub room: Vec<Vec<u8>>
}

impl ObjectToSprite for Place {
    fn get_world_origin(&self) -> Vec<Sprite> {
        self.room.iter()
            .enumerate()
            .map(|(i, row)| row.iter()
                .enumerate()
                .map(|(j, _col)| (i, j))
                .collect::<Vec<(usize, usize)>>())
            .flatten()
            .map(|(el1, el2)|
                Sprite::new(1, el1 as i32, el2 as i32, Layer::MOVABLES))
            .collect::<Vec<Sprite>>()
    }
}

