use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::menu::Menu;
use crate::inventory::item::Spell;
use crate::pawn::pawn::Pawn;

#[derive(Debug, Clone)]
pub struct AI {
    pub intelligence: i8,
    pub seen_target: Vec<Rc<RefCell<Pawn>>>,
    pub selected_target: Option<Rc<RefCell<Pawn>>>,
    pub name: String,
}

pub fn let_ai_or_human_play<Fh, Fa, T>(pawn: Rc<RefCell<Pawn>>, human_action: Fh, ai_action: Fa) -> T where Fh: Fn() -> T, Fa: Fn() -> T {
    if pawn.borrow().playable {
        human_action()
    } else {
        ai_action()
    }
}

impl AI {
    pub fn get_action(&mut self, self_ai: Rc<RefCell<Pawn>>, menu: &Menu) -> std::io::Result<usize> {
        menu.write_line(format!("{} select target", self.seen_target.len()).as_str())?;

        if self.seen_target.is_empty() {
            return Ok(3); // Watch action
        }

        if let None = self.selected_target {
            self.select_target(self_ai.clone(), menu)?;
        }
        //TODO Check if needs to equip weapon
        if let Some(_selected_target) = self.selected_target.clone() {
            return Ok(1); //Attack
        }

        menu.write_line(format!("{} choose what to do next.", self.name).as_str())?;
        Ok(1)
    }

    pub fn select_weapon_or_spell(&self, self_ai: Rc<RefCell<Pawn>>) -> usize {
        let clona_ai = self_ai.clone();
        if !clona_ai.borrow()
            .spell
            .iter()
            .filter(|s| s.mana < clona_ai.borrow().mana)
            .map(|spell| spell.clone())
            .collect::<Vec<Rc<Spell>>>()
            .is_empty() {
            return 1;
        }
        0
    }

    pub fn add_target_to_watched_target(&mut self, target: Rc<RefCell<Pawn>>) {
        self.seen_target.push(target);
    }

    pub fn select_target(&mut self, self_ai: Rc<RefCell<Pawn>>, menu: &Menu) -> std::io::Result<()> {
        //TODO Select best target
        let id = self_ai.clone().borrow().id.clone();
        let local_self = &self;
        let selected_target = local_self.seen_target.iter()
            .filter(|target| {
                let target_clone = target.clone();
                target_clone.borrow().id != id && target_clone.borrow().playable
            })
            .take(1)
            .map(|el| el.clone())
            .collect::<Vec<Rc<RefCell<Pawn>>>>();

        let selected_target = selected_target.get(0).unwrap();
        self.selected_target = Some(selected_target.clone());

        menu.write_line(format!("{} focus on {}", self.name, selected_target.clone().borrow().name).as_str())?;
        Ok(())
    }
}