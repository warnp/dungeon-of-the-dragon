use std::cell::RefCell;
use std::rc::Rc;
use crate::ai::ai::AI;
use crate::environment::world::{Place, Weather, World};
use crate::inventory::item::{DamageTypeEnum, Item, ItemAttackTypeEnum, PartToEquiEnum, Spell};
use crate::inventory::item::DamageTypeEnum::{BLUNT, SLASHING};
use crate::pawn::pawn::{Characteristics, Pawn, Position};
use crate::services::dice::Dice;

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

    pub fn init(weathers: &Vec<Rc<Weather>>, player: Rc<RefCell<Pawn>>, items: &mut Vec<Item>) -> World {
        let mut pawns = Self::generate_non_player_pawns(items);
        pawns.push(player);
        // pawns.push(player);
        World {
            name: "totoland".to_string(),
            places: vec![
                Place {
                    id: 10,
                    name: "La comté".to_string(),
                    weather: Rc::clone(weathers.get(0).unwrap()),
                    time: "Day".to_string(),
                    light: 255,
                    adjacent_places: vec![11],
                    pawns,
                    room: vec![vec![0,0,11,0,0,0],
                               vec![0,0,0 ,0,0,0],
                               vec![0,0,0 ,0,0,0],
                               vec![0,0,0 ,0,0,0],
                               vec![0,0,0 ,0,0,0]]
                },
                Place {
                    id: 11,
                    name: "Pays de Dun".to_string(),
                    weather: Rc::clone(weathers.get(0).unwrap()),
                    time: "Day".to_string(),
                    light: 255,
                    adjacent_places: vec![10],
                    pawns: vec![],
                    room: vec![vec![0,0,0 ,0,0,0],
                               vec![0,0,0 ,0,0,0],
                               vec![0,0,0 ,0,0,0],
                               vec![0,0,0 ,0,0,0],
                               vec![0,0,10,0,0,0]]
                }
            ],
            day: 0,
        }
    }

    pub fn generate_items() -> Vec<Item> {
        vec![ Item {
            name: "Basic iron sword".to_string(),
            damages: || Dice::roll_1d4() as u8,
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
            part_to_equip: PartToEquiEnum::RIGHT_HAND,
            armor_point: 0,
            attack_type: Some(ItemAttackTypeEnum::CONTACT),
        },
            Item {
                name: "Basic wood club".to_string(),
                damages: || Dice::roll_1d4() as u8,
                requirements: Characteristics {
                    force: 1,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 0,
                    willpower: 0,
                    charisma: 0,
                },
                resistances: None,
                power_up: None,
                damages_type: Some(BLUNT),
                part_to_equip: PartToEquiEnum::RIGHT_HAND,
                armor_point: 0,
                attack_type: Some(ItemAttackTypeEnum::CONTACT),
            }
        ]
    }

    fn generate_non_player_pawns(item: &mut Vec<Item>) -> Vec<Rc<RefCell<Pawn>>> {
        let x = (rand::random::<f32>() * 1.0).ceil() as u8;

        std::iter::repeat_with(|| {
            let mut pawn = Pawn {
                id: idgenerator::IdInstance::next_id(),
                name: "bad".to_string(),
                life: 100,
                mana: 0,
                characteristics: Characteristics {
                    force: 5,
                    dexterity: 1,
                    constitution: 1,
                    intelligence: 0,
                    willpower: 0,
                    charisma: 0,
                },
                inventory: vec![Rc::new(item.remove(0))],
                equipped: Default::default(),
                spell: vec![],
                race: "Goblin".to_string(),
                playable: false,
                ai: Rc::new(RefCell::new(Some(AI {
                    intelligence: 0,
                    selected_target: None,
                    seen_target: vec![],
                    name: "bad".to_string(),
                }))),
                position: Position { x: 4, y: 4 },
            };

            pawn.equipped.right_hand = Some(pawn.inventory.get(0).unwrap().clone());
            Rc::new(RefCell::new(pawn))
        })
            .take(x as usize)
            .collect::<Vec<Rc<RefCell<Pawn>>>>()
    }

    pub fn generate_spells() -> Vec<Rc<Spell>> {
        vec![Rc::new(Spell {
            name: "Fireball".to_string(),
            damages: || (Dice::roll_1d4() + Dice::roll_1d4()) as u8,
            mana: 20,
            passive: false,
            requirements: Characteristics {
                force: 0,
                dexterity: 0,
                constitution: 0,
                intelligence: 2,
                willpower: 0,
                charisma: 0,
            },
            effect_time_turns: 0,
            resistances: None,
            power_up: None,
            damages_type: Some(DamageTypeEnum::FIRE),
            attack_type: Some(ItemAttackTypeEnum::MAGIC),
        })]
    }
}