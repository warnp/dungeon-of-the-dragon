use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::cell::RefCell;
use std::rc::Rc;
use console::Term;
use std::sync::mpsc::{Sender, Receiver};
use crate::gui::menu::Menu;
use crate::inventory::item::Pocketable;
use crate::pawn::pawn::Pawn;
use crate::services::dice::{Dice, RollDiceResult};

pub struct Attack;

impl Attack{

    pub fn select_item_to_attack_with( player: Rc<RefCell<Pawn>>, action: Option<usize>, menu: &Menu) -> std::io::Result<Option<Rc<dyn Pocketable>>> {
        return if let Some(act) = action {
            match act {
                0 => Self::get_weapon(player.clone()),
                1 => Self::get_spell(player.clone(), menu),
                _ => Ok(None)
            }
        } else {
            return Ok(None);
        };
    }

    fn get_weapon(player: Rc<RefCell<Pawn>>) -> std::io::Result<Option<Rc<dyn Pocketable>>> {
        let option = player.borrow().equipped.right_hand.clone();
        if let Some(item) = option {
            Ok(Some(item.clone()))
        } else {
            Ok(None)
        }
    }

    fn get_spell( player: Rc<RefCell<Pawn>>, menu: &Menu) -> std::io::Result<Option<Rc<dyn Pocketable>>> {
        let borrowed_player = player.borrow();
        if borrowed_player.spell.is_empty() {
            menu.write_line("You have no spell to cast.")?;
            return Ok(None);
        }

        //Select spell
        return if let Some(s) = menu.menu(borrowed_player
                            .spell
                            .iter()
                            .map(|x| x.get_name().to_string())
                            .collect::<Vec<String>>())? {
            Ok(Some(borrowed_player.spell.get(s).unwrap().clone()))
        } else {
            menu.write_line("You have no spell to cast.")?;
            Ok(None)
        };
    }

    pub fn roll_attack() -> RollDiceResult {
        let dice = Dice::roll_1d20();

        return if dice == 1 {
            RollDiceResult::Fumble
        } else if dice == 20 {
            RollDiceResult::Critical
        } else {
            RollDiceResult::Normal(dice as u8)
        };
    }
}