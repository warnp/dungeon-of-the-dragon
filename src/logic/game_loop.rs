use std::cell::RefCell;
use std::iter::Filter;
use std::rc::{Rc, Weak};
use std::slice::Iter;
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::environment::world::World;
use crate::interact::Actions::Actions;
use crate::pawn::pawn::Pawn;

pub struct GameLoop;

impl GameLoop {
    pub fn iterate(world: World) -> std::io::Result<()> {
        let stdout = Term::stdout();

        //Travel threw places
        loop {
            let world_current_place = world.places.get(0).unwrap();
            stdout.write_line(&format!("You arrived in {}", world_current_place.name)).unwrap();
            let creatures = world_current_place.pawns.iter()
                .filter(|&p| !p.borrow().playable)
                .collect::<Vec<&Rc<RefCell<Pawn>>>>();

            let player = *world_current_place.pawns.iter()
                .filter(|&p| p.borrow().playable)
                .collect::<Vec<&Rc<RefCell<Pawn>>>>()
                .get(0)
                .unwrap();

            loop {
                let creatures_count = creatures.clone().len();
                stdout.write_line(&format!("There is {} creatures here : {}",
                                           creatures_count,
                                           creatures.clone()
                                               .iter()
                                               .map(|&c| c.borrow().name.clone())
                                               .collect::<Vec<String>>()
                                               .join(",\n")
                ))?;

                Self::handle_actions(&stdout, creatures.clone(), player.clone())?;

                stdout.clear_line()?;
            }
        }

        Ok(())
    }

    fn handle_actions(stdout: &Term, creatures: Vec<&Rc<RefCell<Pawn>>>, player: Rc<RefCell<Pawn>>) -> std::io::Result<()> {
        //Select action
        let actions = Select::with_theme(&ColorfulTheme::default())
            .items(&Actions::vec_string())
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        if let Some(action) = actions {
            return match action.into() {
                Actions::USE => Ok(()),
                Actions::WATCH => Ok(()),
                Actions::WALK_TO => Ok(()),
                Actions::ATTACK => Self::attack_action(stdout, creatures),
                Actions::OPEN => Ok(()),
                Actions::EQUIP => {
                    let mut item1= None;
                    {
                        let borrowed_player = player.borrow();
                        let mut inventory = borrowed_player
                            .inventory
                            .iter()
                            .map(|item| item.name.as_str())
                            .collect::<Vec<&str>>();

                        inventory.push("Unequip");
                        let items = Select::with_theme(&ColorfulTheme::default())
                            .items(&inventory)
                            .default(0)
                            .interact_on_opt(&Term::stderr())?;

                        if let Some(i) = items {
                            let selected_item = inventory.get(i).unwrap();
                            if *selected_item == "Unequip" {
                                item1 = None;
                            }else {
                                item1 = Some(borrowed_player.inventory.get(i).unwrap()));
                            }
                        }
                    }

                    if let Some(item) = item1 {
                        player.borrow_mut().equip(item.clone());
                        stdout.write_line(&format!("You equipped {}", rc.clone().name))?;
                    }else{
                        player.borrow_mut().de_equip();
                    }
                    // player.equip()
                    Ok(())
                },
            };
        }
        Ok(())
    }

    fn attack_action(stdout: &Term, creatures: Vec<&Rc<RefCell<Pawn>>>) -> std::io::Result<()> {
        let attackable_things = creatures.clone()
            .iter()
            .map(|&c| c.borrow().name.clone())
            .collect::<Vec<String>>();

        if attackable_things.is_empty() {
            stdout.write_line("There is nothing to attack")?;
            return Ok(());
        }

        stdout.write_line("Attack what?")?;

        //Select action
        let target = Select::with_theme(&ColorfulTheme::default())
            .items(&attackable_things)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        let selected_creature = {
            if let Some(t) = target {
                creatures.get(t).unwrap().clone()
            } else {
                creatures.get(0).unwrap().clone()
            }
        };

        stdout.write_line("whith ?")?;

        //Select action
        let target = Select::with_theme(&ColorfulTheme::default())
            .items(&["your equipped weapon", "a spell"])
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        return Ok(());
    }
}