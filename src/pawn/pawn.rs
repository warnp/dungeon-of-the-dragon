use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;
use crate::ai::ai::AI;
use crate::gui::graphical::sprite::{Layer, ObjectToSprite, Sprite};
use crate::gui::menu::Menu;
use crate::inventory::item::{Item, ItemAttackTypeEnum, PartToEquiEnum, Pocketable, Spell};

pub const CA: u8 = 10;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default)]
pub struct Characteristics {
    pub force: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub willpower: u8,
    pub charisma: u8,
}

impl Add for Characteristics {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            force: self.force + rhs.force,
            dexterity: self.dexterity + rhs.dexterity,
            constitution: self.constitution + rhs.constitution,
            intelligence: self.intelligence + rhs.intelligence,
            willpower: self.willpower + rhs.willpower,
            charisma: self.charisma + rhs.charisma,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone)]
pub struct Pawn {
    pub id: i64,
    pub name: String,
    pub life: u8,
    pub mana: u8,
    pub characteristics: Characteristics,
    pub inventory: Vec<Rc<Item>>,
    pub equipped: EquipablePart,
    pub spell: Vec<Rc<Spell>>,
    pub race: String,
    pub playable: bool,
    pub ai: Rc<RefCell<Option<AI>>>,
    pub position: Position,
}

impl Pawn {
    pub fn hit(&self, item: Rc<dyn Pocketable>, target: Rc<RefCell<Pawn>>) -> u8 {
        target.clone().borrow_mut().take_hit(item.clone().get_damages())
    }

    pub fn take_hit(&mut self, damage: u8) -> u8 {
        self.life -= damage;
        damage
    }

    pub fn equip(&mut self, item: Rc<Item>) {
        match item.clone().part_to_equip {
            PartToEquiEnum::HEAD => self.equipped.head = Some(item.clone()),
            PartToEquiEnum::BODY => self.equipped.body = Some(item.clone()),
            PartToEquiEnum::LEFT_HAND => self.equipped.left_hand = Some(item.clone()),
            PartToEquiEnum::RIGHT_HAND => self.equipped.right_hand = Some(item.clone()),
            PartToEquiEnum::LEGS => self.equipped.legs = Some(item.clone()),
            PartToEquiEnum::FEET => self.equipped.feet = Some(item.clone()),
        }
    }

    pub fn de_equip(&mut self, part_to_unequip: PartToEquiEnum) {
        match part_to_unequip {
            PartToEquiEnum::HEAD => self.equipped.head = None,
            PartToEquiEnum::BODY => self.equipped.body = None,
            PartToEquiEnum::LEFT_HAND => self.equipped.left_hand = None,
            PartToEquiEnum::RIGHT_HAND => self.equipped.right_hand = None,
            PartToEquiEnum::LEGS => self.equipped.legs = None,
            PartToEquiEnum::FEET => self.equipped.feet = None,
        }
    }

    pub fn calculate_power_up(&self) -> Option<Characteristics> {
        self.equipped.get_all_props()
            .iter()
            .map(|el|
                if let Some(item) = el.1 {
                    if let Some(power_up) = item.power_up {
                        power_up
                    } else {
                        Characteristics::default()
                    }
                } else {
                    Characteristics::default()
                })
            .reduce(|acc, el|
                acc + el
            )
    }

    pub fn calculate_usability(&self, damage_dealer_pocketable: Rc<dyn Pocketable>, menu: &Menu) -> std::io::Result<u8> {

        //Calculate total characteristics
        let charac = self.characteristics + self.calculate_power_up().unwrap();

        let item_clone = damage_dealer_pocketable.clone();

        let mana = {
            if let Some(attack_type) = item_clone.get_attack_type() {
                match attack_type {
                    ItemAttackTypeEnum::CONTACT | ItemAttackTypeEnum::DISTANCE => None,
                    ItemAttackTypeEnum::MAGIC => Some(self.mana)
                }
            } else {
                None
            }
        };

        let usability = item_clone.calculate_usability(&charac, mana);
        let pocketable_name = item_clone.get_name().to_string();

        menu.write_line(format!("usability {}, charac {:#?}, pocketable charac {:#?}", usability, charac, item_clone.get_characteristics()).as_str())?;
        if usability > 127 {
            menu.write_line(&format!("Good use of {}", pocketable_name))?;
            Ok(usability)
        } else if usability > 0 {
            menu.write_line(&format!("Average use of {}", pocketable_name))?;
            Ok(usability)
        } else {
            menu.write_line(&format!("You don't know how to use {}", pocketable_name))?;
            Ok(0)
        }
    }

    pub fn calculate_armor_points(&self) -> u8 {
        let total_armor = self.pure_armor_points();

        if total_armor > 0 {
            total_armor + CA
        } else {
            self.dex_total() + CA
        }
    }

    fn dex_total(&self) -> u8 {
        let dex_total = self.calculate_power_up()
            .iter()
            .map(|char| char.dexterity)
            .reduce(|acc, e| acc + e)
            .unwrap();
        dex_total
    }

    fn pure_armor_points(&self) -> u8 {
        let total_armor = self.equipped.get_all_props()
            .iter()
            .map(|el| el.1)
            .filter(|&&el| {
                if let Some(item) = el {
                    return item.clone().armor_point > 0;
                }
                return false;
            }
            )
            .map(|el| if let Some(e) = el {
                e.clone().armor_point
            } else {
                0
            })
            .reduce(|acc, el| acc + el)
            .unwrap_or(0);
        total_armor
    }
}

impl ObjectToSprite for Pawn {
    fn get_world_origin(&self) -> Vec<Sprite> {
        let texture_id = match self.race.as_str() {
            "human" => 200,
            "goblin" => 201,
            _ => 201
        };

        vec![Sprite::new(texture_id, self.position.x as i32, self.position.y as i32, Layer::MOVABLES)]
    }
}

#[derive(Debug, Clone, Default)]
pub struct EquipablePart {
    pub head: Option<Rc<Item>>,
    pub right_hand: Option<Rc<Item>>,
    pub left_hand: Option<Rc<Item>>,
    pub body: Option<Rc<Item>>,
    pub legs: Option<Rc<Item>>,
    pub feet: Option<Rc<Item>>,
}

impl EquipablePart {
    pub fn get_all_props(&self) -> HashMap<PartToEquiEnum, &Option<Rc<Item>>> {
        let mut result_map = HashMap::new();
        result_map.insert(PartToEquiEnum::BODY, &self.body);
        result_map.insert(PartToEquiEnum::RIGHT_HAND, &self.right_hand);
        result_map.insert(PartToEquiEnum::LEFT_HAND, &self.left_hand);
        result_map.insert(PartToEquiEnum::HEAD, &self.head);
        result_map.insert(PartToEquiEnum::LEGS, &self.legs);
        result_map.insert(PartToEquiEnum::FEET, &self.feet);

        result_map
    }
}
