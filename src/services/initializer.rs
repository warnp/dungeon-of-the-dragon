use std::borrow::Borrow;
use std::rc::Rc;
use crate::environment::world::{Place, Weather, World};
use crate::pawn::pawn::Pawn;

pub struct Initializer;

impl Initializer {
    pub fn init_weather() -> Vec<Rc<Weather>> {
        vec![
            Rc::new(Weather{
                name: "Sun".to_string(),
                temperature: 20,
                humidity: 30,
                visibility: 255,
                wind: 0,
            }),
            Rc::new(Weather{
                name: "Windy".to_string(),
                temperature: 15,
                humidity: 30,
                visibility: 255,
                wind: 100,
            }),
            Rc::new(Weather{
                name: "Rain".to_string(),
                temperature: 10,
                humidity: 90,
                visibility: 127,
                wind: 20,
            }),
            Rc::new(Weather{
                name: "Fog".to_string(),
                temperature: 10,
                humidity: 90,
                visibility: 32,
                wind: 0,
            }),
        ]
    }

    pub fn init(weathers: &Vec<Rc<Weather>>,player: Pawn) -> World {

        World {
            name: "totoland".to_string(),
            places: vec![
                Place{
                    name: "La comté".to_string(),
                    weather: weathers.get(0).unwrap().clone(),
                    time: "Day".to_string(),
                    light: 255,
                    adjacent_places: vec![],
                    pawns: vec![],
                }
            ],
            day: 0,
        }

    }
}