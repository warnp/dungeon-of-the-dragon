use std::cell::{Ref, RefCell, RefMut};
use std::io::{Error, stdout, Write};
use std::iter::Filter;
use std::rc::{Rc, Weak};
use std::slice::Iter;
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::environment::world::World;
use crate::interact::Actions::Actions;
use crate::inventory::item::{Item, Pocketable, Spell};
use crate::menu;
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
                .filter(|&p| {
                    return !p.borrow().playable;
                })
                .collect::<Vec<&Rc<RefCell<Pawn>>>>();

            let player = *world_current_place.pawns.iter()
                .filter(|&p| {
                    p.borrow().playable
                })
                .collect::<Vec<&Rc<RefCell<Pawn>>>>()
                .get(0)
                .unwrap();

            loop {
                let creatures_count = creatures.clone().len();
                stdout.write_line(&format!("There is {} creatures here : {}",
                                           creatures_count,
                                           creatures.clone()
                                               .iter()
                                               .map(|&c| {
                                                   c.borrow().name.clone()
                                               })
                                               .collect::<Vec<String>>()
                                               .join(", ")
                ))?;

                Self::handle_actions(&stdout, creatures.clone(), player.clone())?;

                stdout.clear_line()?;
            }
        }

        Ok(())
    }

    fn handle_actions(stdout: &Term, creatures: Vec<&Rc<RefCell<Pawn>>>, player: Rc<RefCell<Pawn>>) -> std::io::Result<()> {
        //Select action
        let actions = menu!(Actions::vec_string());

        if let Some(action) = actions {
            return match action.into() {
                Actions::USE => Ok(()),
                Actions::WATCH => Ok(()),
                Actions::WALK_TO => Ok(()),
                Actions::ATTACK => Self::attack_action(stdout, creatures, player.clone()),
                Actions::OPEN => Ok(()),
                Actions::EQUIP => Self::equip_item(stdout, player)
            };
        }
        Ok(())
    }

    fn equip_item(stdout: &Term, player: Rc<RefCell<Pawn>>) -> Result<(), Error> {
        let mut item1: Option<Item> = None;
        {
            let borrowed_player = player.borrow();

            let mut inventory = borrowed_player
                .inventory
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<&str>>();

            inventory.push("Unequip");

            let items = menu!(inventory);

            if let Some(i) = items {
                let selected_item = inventory.get(i).unwrap();
                if *selected_item == "Unequip" {
                    item1 = None;
                } else {
                    item1 = Some(borrowed_player.inventory.get(i).unwrap().to_owned());
                }
            }
        }

        let mut mutable_player = player.borrow_mut();
        if let Some(item) = item1 {
            stdout.write_line(&format!("You equipped {}", item.clone().name))?;
            mutable_player.equip(Rc::new(item));
        } else {
            mutable_player.de_equip();
        }
        Ok(())
    }

    fn attack_action(stdout: &Term, creatures: Vec<&Rc<RefCell<Pawn>>>, player: Rc<RefCell<Pawn>>) -> std::io::Result<()> {
        let attackable_things = creatures.clone()
            .iter()
            .map(|&c| {
                c.borrow().name.clone()
            })
            .collect::<Vec<String>>();

        //TODO Add attackable element if any

        if attackable_things.is_empty() {
            stdout.write_line("There is nothing to attack")?;
            return Ok(());
        }

        stdout.write_line("Attack what?")?;

        //Select action
        let target = menu!(attackable_things);

        let selected_creature = {
            if let Some(t) = target {
                creatures.get(t).unwrap().clone()
            } else {
                //Select default creature
                creatures.get(0).unwrap().clone()
            }
        };

        stdout.write_line("whith ?")?;

        //Select action
        let action = menu!(["your equipped weapon", "a spell"]);

        let damages_willing_to_deal = {
            if let Some(act) = action {
                match act {
                    0 => (),
                    1 => {
                        let borrowed_player = player.borrow();
                        if borrowed_player.spell.is_empty() {
                            stdout.write_line("You have no spell to cast.")?;
                            return Ok(());
                        }
                        //Select spell
                        if let Some(s) = menu!(borrowed_player
                            .spell
                            .iter()
                            .map(|x| x.get_name())
                            .collect::<Vec<&str>>()) {
                            let selected_spell = borrowed_player.spell.get(s).unwrap().clone();

                            Self::calculate_damage_to_deal(selected_spell.clone(), player.clone(), stdout)?;



                        } else {
                            stdout.write_line("You have no spell to cast.")?;
                        }
                    }
                    _ => ()
                }
            }
        };
        player.borrow().hit(10, selected_creature.clone());

        return Ok(());
    }


    fn calculate_damage_to_deal(damage_dealer_pocketable: Rc<dyn Pocketable>, player: Rc<RefCell<Pawn>>, stdout: &Term) -> std::io::Result<u8> {
        let borrowed_player = player.borrow();

        //Calculate total characteristics
        let charac = {
            if let Some(equipped_item) = borrowed_player.clone().equipped {
                if let Some(power_up) = equipped_item.power_up {
                    borrowed_player.characteristics + power_up
                }else {
                    borrowed_player.characteristics
                }
            }else {
                borrowed_player.characteristics
            }
        };

        let usability = damage_dealer_pocketable.clone().calculate_usability(&charac, Some(borrowed_player.mana));
        let pocketable_name = damage_dealer_pocketable.clone().get_name().to_string();

        let random1 = rand::random::<u8>();


        if usability > 127 {
            stdout.write_line(&format!("Good use of {}", pocketable_name))?;

        } else if usability > 0 {
            stdout.write_line(&format!("Average use of {}", pocketable_name))?;
        } else {
            stdout.write_line(&format!("You don't know how to use {}", pocketable_name))?;
        }
        Ok(0)
    }
}