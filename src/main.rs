use std::cell::RefCell;
use std::fmt::format;
use std::io::{stdout, Write};
use std::rc::Rc;
use crate::inventory::item::DamageTypeEnum::SLASHING;
use crate::inventory::item::Item;
use crate::pawn::pawn::{Characteristics, Pawn};
use crate::services::initializer::Initializer;

use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::interact::Actions::Actions;
use crate::logic::game_loop::GameLoop;

mod pawn;
mod inventory;
mod environment;
mod services;
mod interact;
mod logic;
mod gui;

fn main() -> std::io::Result<()> {
    let sword = Item {
        name: "Basic iron sword".to_string(),
        damages: 2,
        requirements: Characteristics {
            force: 2,
            dexterity: 1,
            constitution: 0,
            intelligence: 0,
            willpower: 0,
            charisma: 0,
        },
        resistances: None,
        power_up: None,
        damages_type: Some(SLASHING),
    };

    let player1 = Rc::new(RefCell::new(Pawn {
        name: "Toto".to_string(),
        life: 100,
        spell: vec![],
        race: "Humain".to_string(),
        inventory: vec![sword],
        mana: 100,
        characteristics: Characteristics {
            force: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            willpower: 0,
            charisma: 0,
        },
        playable: true,
        equipped: None,
    }));

    let weather_list = Initializer::init_weather();
    let world = Initializer::init(&weather_list, player1.clone());

    GameLoop::iterate(world)
}
