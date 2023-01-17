use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::environment::world::{Place, Weather, World};
use crate::pawn::pawn::{Characteristics, Pawn};

pub struct Initializer;

impl Initializer {
    pub fn init_weather() -> Vec<Rc<Weather>> {
        vec![
            Rc::new(Weather {
                name: "Sun".to_string(),
                temperature: 20,
                humidity: 30,
                visibility: 255,
                wind: 0,
            }),
            Rc::new(Weather {
                name: "Windy".to_string(),
                temperature: 15,
                humidity: 30,
                visibility: 255,
                wind: 100,
            }),
            Rc::new(Weather {
                name: "Rain".to_string(),
                temperature: 10,
                humidity: 90,
                visibility: 127,
                wind: 20,
            }),
            Rc::new(Weather {
                name: "Fog".to_string(),
                temperature: 10,
                humidity: 90,
                visibility: 32,
                wind: 0,
            }),
        ]
    }

    pub fn init(weathers: &Vec<Rc<Weather>>, player: Rc<RefCell<Pawn>>) -> World {
        let mut pawns = Self::generate_non_player_pawns();
        pawns.push(player);
        // pawns.push(player);
        World {
            name: "totoland".to_string(),
            places: vec![
                Place {
                    name: "La comté".to_string(),
                    weather: Rc::clone(weathers.get(0).unwrap()),
                    time: "Day".to_string(),
                    light: 255,
                    adjacent_places: vec![],
                    pawns,
                }
            ],
            day: 0,
        }
    }

    fn generate_non_player_pawns() -> Vec<Rc<RefCell<Pawn>>> {
        let x = (rand::random::<f32>() * 4.0).floor() as u8;

        std::iter::repeat_with(|| Rc::new(RefCell::new( Pawn {
            name: "bad".to_string(),
            life: 100,
            mana: 0,
            characteristics: Characteristics {
                force: 1,
                dexterity: 1,
                constitution: 1,
                intelligence: 0,
                willpower: 0,
                charisma: 0,
            },
            inventory: vec![],
            equipped: None,
            spell: vec![],
            race: "Goblin".to_string(),
            playable: false,
        })))
            .take(x as usize)
            .collect::<Vec<Rc<RefCell<Pawn>>>>()
    }
}