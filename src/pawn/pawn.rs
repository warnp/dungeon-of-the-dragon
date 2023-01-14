use std::rc::Rc;
use crate::inventory::item::{Item, Spell};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Characteristics {
    pub force: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub willpower: u8,
    pub charisma: u8,
}

#[derive(Debug)]
pub struct Pawn {
    pub name: String,
    pub life: u8,
    pub mana: u8,
    pub characteristics: Characteristics,
    pub inventory: Vec<Rc<Item>>,
    pub spell: Vec<Rc<Spell>>,
    pub race: String,
    pub playable: bool,
}

impl Pawn {
    fn hit(&self, damage: u8, target: &mut Pawn) {
        target.take_hit(damage)
    }

    fn take_hit(&mut self, damage: u8) {
        println!("{} take {} damages", self.name, damage);
        self.life -= damage;
    }

    fn heal(&self, heal_points: u8, target: &mut Pawn) {
        println!("{} take {} damages", self.name, heal_points);

        target.take_hit(heal_points)
    }
}