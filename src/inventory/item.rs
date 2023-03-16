use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::pawn::pawn::Characteristics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageTypeEnum {
    PIERCING,
    SLASHING,
    BLUNT,
    ELECTRIC,
    FIRE,
    ICE,
    HEAL,
}

#[derive(Debug, Clone)]
pub enum ItemAttackTypeEnum {
    CONTACT,
    DISTANCE,
    MAGIC
}

pub trait Pocketable {
    fn get_damages(&self) -> u8;

    fn get_resistance(&self) -> Option<DamageTypeEnum>;

    fn get_name(&self) -> &str;

    fn get_requirements(&self) -> &Characteristics;

    fn calculate_usability(&self, pawn_charac: &Characteristics, mana: Option<u8>) -> u8 {
        let adjustment_value = 2;
        let mut result = 0;

        let requirements = self.get_requirements();
        let charisma = requirements.charisma;
        if charisma > 0 {
            if charisma < pawn_charac.charisma {
                if charisma + adjustment_value < pawn_charac.charisma {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        let intelligence = requirements.intelligence;
        if intelligence > 0 {
            if intelligence < pawn_charac.intelligence {
                if intelligence + adjustment_value < pawn_charac.intelligence {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        let willpower = requirements.willpower;
        if willpower > 0 {
            if willpower < pawn_charac.willpower {
                if willpower + adjustment_value < pawn_charac.willpower {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        let force = requirements.force;
        if force > 0 {
            if force < pawn_charac.force {
                if force + adjustment_value < pawn_charac.force {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        let dexterity = requirements.dexterity;
        if dexterity > 0 {
            if dexterity < pawn_charac.dexterity {
                if dexterity + adjustment_value < pawn_charac.dexterity {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        let constitution = requirements.constitution;
        if constitution > 0 {
            if constitution < pawn_charac.constitution {
                if constitution + adjustment_value < pawn_charac.constitution {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        result
    }

    fn get_power_up(&self) -> &Option<Characteristics>;

    fn get_damage_type(&self) -> Option<DamageTypeEnum>;

    fn get_attack_type(&self) -> Option<ItemAttackTypeEnum>;

    fn get_characteristics(&self) -> Characteristics;

    fn get_range(&self) -> Option<u16>;

}
#[warn(non_camel_case_types)]
#[derive(Debug, Clone,Eq, Hash, PartialEq)]
pub enum PartToEquiEnum {
    HEAD,
    RIGHT_HAND,
    LEFT_HAND,
    BODY,
    LEGS,
    FEET
}

impl Display for PartToEquiEnum{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub damages: fn() -> u8,
    pub requirements: Characteristics,
    pub resistances: Option<DamageTypeEnum>,
    pub power_up: Option<Characteristics>,
    pub damages_type: Option<DamageTypeEnum>,
    pub part_to_equip: PartToEquiEnum,
    pub armor_point: u8,
    pub attack_type: Option<ItemAttackTypeEnum>,
    pub range: Option<u16>
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub damages: fn() -> u8,
    pub mana: u8,
    pub passive: bool,
    pub requirements: Characteristics,
    pub effect_time_turns: u8,
    pub resistances: Option<DamageTypeEnum>,
    pub power_up: Option<Characteristics>,
    pub damages_type: Option<DamageTypeEnum>,
    pub attack_type: Option<ItemAttackTypeEnum>,
    pub range: Option<u16>
}

impl Pocketable for Item {

    fn get_damages(&self) -> u8 {
        (self.damages)()
    }

    fn get_resistance(&self) -> Option<DamageTypeEnum> {
        self.resistances.clone()
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_requirements(&self) -> &Characteristics {
        &self.requirements
    }

    fn get_power_up(&self) -> &Option<Characteristics> {
        &self.power_up
    }

    fn get_damage_type(&self) -> Option<DamageTypeEnum> {
        self.damages_type.clone()
    }

    fn get_attack_type(&self) -> Option<ItemAttackTypeEnum> {
        self.attack_type.clone()
    }

    fn get_characteristics(&self) -> Characteristics {
        self.requirements
    }

    fn get_range(&self) -> Option<u16> {
        self.range
    }
}

impl Pocketable for Spell {

    fn get_characteristics(&self) -> Characteristics {
        self.requirements
    }

    fn get_damages(&self) -> u8 {
        (self.damages)()
    }

    fn get_resistance(&self) -> Option<DamageTypeEnum> {
        self.resistances.clone()
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_power_up(&self) -> &Option<Characteristics> {
        &self.power_up
    }

    fn get_damage_type(&self) -> Option<DamageTypeEnum> {
        self.damages_type.clone()
    }

    fn get_attack_type(&self) -> Option<ItemAttackTypeEnum> {
        self.attack_type.clone()
    }

    fn get_requirements(&self) -> &Characteristics {
        &self.requirements
    }

    fn get_range(&self) -> Option<u16> {
        self.range
    }
}

