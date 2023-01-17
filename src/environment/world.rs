use std::cell::RefCell;
use std::rc::Rc;
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
    pub name: String,
    pub weather: Rc<Weather>,
    pub time: String,
    pub light: u8,
    pub adjacent_places: Vec<Rc<Place>>,
    pub pawns: Vec<Rc<RefCell<Pawn>>>,
}

