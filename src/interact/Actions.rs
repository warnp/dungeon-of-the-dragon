use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Actions {
    OPEN = 0,
    ATTACK,
    WALK_TO,
    WATCH,
    USE,
    EQUIP
}

impl Actions {
    pub fn vec_string() -> Vec<String> {
        vec![Actions::OPEN.to_string(),
             Actions::ATTACK.to_string(),
             Actions::WALK_TO.to_string(),
             Actions::WATCH.to_string(),
             Actions::USE.to_string(),
             Actions::EQUIP.to_string()]
    }

}

impl From<usize> for Actions {
    fn from(value: usize) -> Self {
        match value {
            x if x == Actions::OPEN as usize => Actions::OPEN,
            x if x == Actions::ATTACK as usize => Actions::ATTACK,
            x if x == Actions::USE as usize => Actions::USE,
            x if x == Actions::WALK_TO as usize => Actions::WALK_TO,
            x if x == Actions::WATCH as usize => Actions::WATCH,
            x if x == Actions::EQUIP as usize => Actions::EQUIP,
            _ => Actions::OPEN,
        }
    }
}

impl Display for Actions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}