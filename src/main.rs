use crate::pawn::pawn::{Characteristics, Pawn};
use crate::services::initializer::Initializer;

mod pawn;
mod inventory;
mod environment;
mod services;

fn main() {
    let player1 = Pawn {
        name: "Toto".to_string(),
        life: 100,
        spell: vec![],
        race: "Humain".to_string(),
        inventory: vec![],
        mana: 100,
        characteristics: Characteristics{
            force: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            willpower: 0,
            charisma: 0,
        },
        playable: true,
    };

    let weather_list = Initializer::init_weather();
    let world = Initializer::init(&weather_list, player1);

    println!("Hello, world!");
}
