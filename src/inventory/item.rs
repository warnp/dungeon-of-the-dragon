use std::fmt::{Display, Formatter};
use crate::pawn::pawn::Characteristics;

#[derive(Debug, Clone)]
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

    fn calculate_usability(&self, pawn_charac: &Characteristics, mana: Option<u8>) -> u8;

    fn get_power_up(&self) -> &Option<Characteristics>;

    fn get_damage_type(&self) -> Option<DamageTypeEnum>;

    fn get_attack_type(&self) -> Option<ItemAttackTypeEnum>;

}

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
    pub attack_type: Option<ItemAttackTypeEnum>
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
    pub attack_type: Option<ItemAttackTypeEnum>

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

    fn calculate_usability(&self, pawn_charac: &Characteristics, mana: Option<u8>) -> u8 {
        let adjustment_value = 2;
        let mut result = 0;

        if self.requirements.charisma > 0 {
            if self.requirements.charisma < pawn_charac.charisma {
                if self.requirements.charisma + adjustment_value < pawn_charac.charisma {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.intelligence > 0 {
            if self.requirements.intelligence < pawn_charac.intelligence {
                if self.requirements.intelligence + adjustment_value < pawn_charac.intelligence {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.willpower > 0 {
            if self.requirements.willpower < pawn_charac.willpower {
                if self.requirements.willpower + adjustment_value < pawn_charac.willpower {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.force > 0 {
            if self.requirements.force < pawn_charac.force {
                if self.requirements.force + adjustment_value < pawn_charac.force {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.dexterity > 0 {
            if self.requirements.dexterity < pawn_charac.dexterity {
                if self.requirements.dexterity + adjustment_value < pawn_charac.dexterity {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.constitution > 0 {
            if self.requirements.constitution < pawn_charac.constitution {
                if self.requirements.constitution + adjustment_value < pawn_charac.constitution {
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

    fn get_power_up(&self) -> &Option<Characteristics> {
        &self.power_up
    }

    fn get_damage_type(&self) -> Option<DamageTypeEnum> {
        self.damages_type.clone()
    }

    fn get_attack_type(&self) -> Option<ItemAttackTypeEnum> {
        self.attack_type.clone()
    }
}

impl Pocketable for Spell {
    fn get_damages(&self) -> u8 {
        (self.damages)()
    }

    fn get_resistance(&self) -> Option<DamageTypeEnum> {
        self.resistances.clone()
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn calculate_usability(&self, pawn_charac: &Characteristics, mana: Option<u8>) -> u8 {
        let adjustment_value = 2;
        let mut result = 0;

        if let Some(m) = mana {
            if m < self.mana {
                return 0;
            }
        } else {
            return 0;
        }

        if self.requirements.charisma > 0 {
            if self.requirements.charisma < pawn_charac.charisma {
                if self.requirements.charisma + adjustment_value < pawn_charac.charisma {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.intelligence > 0 {
            if self.requirements.intelligence < pawn_charac.intelligence {
                if self.requirements.intelligence + adjustment_value < pawn_charac.intelligence {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.willpower > 0 {
            if self.requirements.willpower < pawn_charac.willpower {
                if self.requirements.willpower + adjustment_value < pawn_charac.willpower {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.force > 0 {
            if self.requirements.force < pawn_charac.force {
                if self.requirements.force + adjustment_value < pawn_charac.force {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.dexterity > 0 {
            if self.requirements.dexterity < pawn_charac.dexterity {
                if self.requirements.dexterity + adjustment_value < pawn_charac.dexterity {
                    result = 255;
                } else {
                    result = 127;
                }
            } else {
                return 0;
            }
        }

        if self.requirements.constitution > 0 {
            if self.requirements.constitution < pawn_charac.constitution {
                if self.requirements.constitution + adjustment_value < pawn_charac.constitution {
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

    fn get_power_up(&self) -> &Option<Characteristics> {
        &self.power_up
    }

    fn get_damage_type(&self) -> Option<DamageTypeEnum> {
        self.damages_type.clone()
    }

    fn get_attack_type(&self) -> Option<ItemAttackTypeEnum> {
        self.attack_type.clone()
    }

}

