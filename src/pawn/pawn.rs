use std::rc::Rc;
use crate::inventory::item::{Item, Spell};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Characteristics {
    pub force: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub willpower: u8,
    pub charisma: u8,
}

#[derive(Debug, Clone)]
pub struct Pawn {
    pub name: String,
    pub life: u8,
    pub mana: u8,
    pub characteristics: Characteristics,
    pub inventory: Vec<Item>,
    pub equipped: Option<Rc<Item>>,
    pub spell: Vec<Rc<Spell>>,
    pub race: String,
    pub playable: bool,
}

impl Pawn {
    pub fn hit(&self, damage: u8, target: &mut Pawn) {
        target.take_hit(damage)
    }

    pub fn take_hit(&mut self, damage: u8) {
        println!("{} take {} damages", self.name, damage);
        self.life -= damage;
    }

    pub fn heal(&self, heal_points: u8, target: &mut Pawn) {
        println!("{} take {} damages", self.name, heal_points);

        target.take_hit(heal_points)
    }

    pub fn equip(&mut self, item: Rc<Item>) {
        self.equipped = Some(item);
    }

    pub fn de_equip(&mut self) {
        self.equipped = None;
    }
}