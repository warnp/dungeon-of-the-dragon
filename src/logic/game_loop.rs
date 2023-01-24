use std::cell::{Ref, RefCell, RefMut};
use std::fmt::format;
use std::io::{Error, stdout, Write};
use std::iter::Filter;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::slice::Iter;
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::environment::world::World;
use crate::interact::Actions::Actions;
use crate::inventory::item::{Item, ItemAttackTypeEnum, PartToEquiEnum, Pocketable, Spell};
use crate::menu;
use crate::pawn::pawn::{Characteristics, Pawn};
use crate::services::{interactions::Attack, dice::Dice};
use crate::services::dice::RollDiceResult;

pub struct GameLoop;

impl GameLoop {
    pub fn iterate(world: World) -> std::io::Result<()> {
        let stdout = Term::stdout();

        //Travel threw places
        loop {
            let world_current_place = world.places.get(0).unwrap();
            stdout.write_line(format!("You arrived in {}", world_current_place.name).as_str()).unwrap();
            // let creatures = world_current_place.pawns.iter()
            //     .filter(|&p| {
            //         return !p.clone().borrow().playable;
            //     })
            //     .map(|e| e.clone())
            //     .collect::<Vec<Rc<RefCell<Pawn>>>>();
            //
            // let player = world_current_place.pawns.iter()
            //     .filter(|&p| {
            //         p.borrow().playable
            //     })
            //     .map(|e| e.clone())
            //     .collect::<Vec<Rc<RefCell<Pawn>>>>()
            //     .get(0)
            //     .unwrap();

            loop {
                let pawns = &world_current_place.pawns;
                let creatures = (&pawns)
                    .iter()
                    .filter(|e| !e.borrow().playable)
                    .map(|c| {
                        c.borrow().name.clone()
                    })
                    .collect::<Vec<String>>();
                let creatures_count = (&creatures).len();
                stdout.write_line(format!("There is {} creatures here : {}",
                                          creatures_count,
                                          creatures.join(", ")
                ).as_str())?;

                Self::handle_actions(&stdout, &Self::order_pawns(pawns)?)?;

                stdout.clear_line()?;
            }
        }
    }

    fn order_pawns(pawns: &Vec<Rc<RefCell<Pawn>>>) -> std::io::Result<Vec<Rc<RefCell<Pawn>>>> {
        let mut x = pawns.clone();
        x.sort_by(|a, b| a.borrow().characteristics.dexterity.cmp(&b.borrow().characteristics.dexterity));
        Ok(x)
    }

    fn handle_actions(stdout: &Term, pawns: &Vec<Rc<RefCell<Pawn>>>) -> std::io::Result<()> {
        let mut pawns_iter = (&pawns).iter();

        while let Some(current_pawn) = pawns_iter.next() {

            //Select action
            let actions = menu!(Actions::vec_string());

            if let Some(action) = actions {
                return match action.into() {
                    Actions::USE => Ok(()),
                    Actions::WATCH => Self::watch_action(stdout, pawns),
                    Actions::WALK_TO => Ok(()),
                    Actions::ATTACK => Self::attack_action(stdout, pawns, current_pawn.clone()),
                    Actions::OPEN => Ok(()),
                    Actions::EQUIP => Self::equip_item(stdout, current_pawn.clone())
                };
            }
        }

        Ok(())
    }

    fn watch_action(stdout: &Term, creatures: &Vec<Rc<RefCell<Pawn>>>) -> Result<(), Error> {
        if creatures.is_empty() {
            stdout.write_line("Nothing to see here")?;
            return Ok(());
        }

        //TODO use perception here
        //TODO Add item in room
        stdout.write_line("What ?")?;

        let creatures_name = creatures.iter()
            .filter(|e| e.clone().borrow().life > 0)
            .map(|e| e.clone().borrow().name.clone())
            .collect::<Vec<String>>();
        let option = menu!(creatures_name);

        if let Some(creature_id) = option {
            let creature = creatures.get(creature_id).unwrap();
            stdout.write_line(format!("{:#?}", creature).as_str())?;
        }

        Ok(())
    }

    fn equip_item(stdout: &Term, player: Rc<RefCell<Pawn>>) -> Result<(), Error> {
        let selected_item: Option<Rc<Item>> =
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
                        None
                    } else {
                        Some(borrowed_player.inventory.get(i).unwrap().clone())
                    }
                } else {
                    None
                }
            };

        let mut mutable_player = player.borrow_mut();
        if let Some(item) = selected_item {
            stdout.write_line(format!("You equipped {}", item.clone().name).as_str())?;
            mutable_player.equip(Rc::clone(&item));
        } else {
            stdout.write_line("What part do you want to unequip?")?;
            let part_to_unequip = menu!([
                PartToEquiEnum::HEAD,
                PartToEquiEnum::FEET,
                PartToEquiEnum::LEGS,
                PartToEquiEnum::RIGHT_HAND,
                PartToEquiEnum::LEFT_HAND,
                PartToEquiEnum::BODY
            ]);

            let equip_enum = {
                if let Some(part) = part_to_unequip {
                    match part {
                        0 => PartToEquiEnum::HEAD,
                        1 => PartToEquiEnum::FEET,
                        2 => PartToEquiEnum::LEGS,
                        3 => PartToEquiEnum::RIGHT_HAND,
                        4 => PartToEquiEnum::LEFT_HAND,
                        5 => PartToEquiEnum::BODY,
                        _ => PartToEquiEnum::BODY,
                    }
                } else {
                    PartToEquiEnum::BODY
                }
            };

            mutable_player.de_equip(equip_enum);
        }
        Ok(())
    }

    fn attack_action(stdout: &Term, creatures: &Vec<Rc<RefCell<Pawn>>>, player: Rc<RefCell<Pawn>>) -> std::io::Result<()> {
        let attackable_things = creatures.clone()
            .iter()
            .filter(|&e| e.borrow().life > 0)
            .map(|c| {
                c.borrow().name.clone()
            })
            .collect::<Vec<String>>();

        //TODO Add attackable element if any

        if attackable_things.is_empty() {
            stdout.write_line("There is nothing to attack")?;
            return Ok(());
        }

        stdout.write_line("whith ?")?;

        //Select action
        let action = menu!(["your equipped weapon", "a spell"]);

        let select_item_to_attack_with = Attack::select_item_to_attack_with(stdout, player.clone(), action)?;

        if let None = select_item_to_attack_with {
            stdout.write_line("You have no way to deal damage to any target!")?;
            return Ok(());
        }

        let unwrapped_selected_item = select_item_to_attack_with.unwrap();
        let usability = player.borrow().calculate_usability(unwrapped_selected_item.clone(), stdout)?;

        if usability == 0 {
            stdout.write_line("You don't know what to do!")?;
            return Ok(());
        }

        stdout.write_line("Attack what?")?;
        let selected_creature = Self::select_target(creatures, attackable_things)?;

        stdout.write_line("Roll 1d20 : ")?;

        // Roll dice
        match Attack::roll_attack() {
            RollDiceResult::Critical => {
                stdout.write_line("Critial!")?;

                let damages_dealt = player.clone().borrow().hit(unwrapped_selected_item.clone(), selected_creature.clone());
                stdout.write_line(format!("{} inflict {} to {}", player.clone().borrow().name, damages_dealt, selected_creature.clone().borrow().name).as_str())?;
            }
            RollDiceResult::Fumble => {
                stdout.write_line("Fumble")?;
            }
            RollDiceResult::Normal(dice_result) => {
                stdout.write_line(format!("Normal attack, dice result : {}", dice_result).as_str())?;

                let target_armor_points = selected_creature.clone().borrow().calculate_armor_points();

                //Add modificator to dice roll
                let dice_result = if let Some(attack_type) = unwrapped_selected_item.clone().get_attack_type() {
                    let characteristics = player.clone().borrow().characteristics;
                    match attack_type {
                        ItemAttackTypeEnum::CONTACT => dice_result + characteristics.force,
                        ItemAttackTypeEnum::DISTANCE => dice_result + characteristics.dexterity,
                        ItemAttackTypeEnum::MAGIC => dice_result + characteristics.willpower,
                    }
                } else {
                    dice_result
                };

                // Check if target CA is greater than dice roll with modificator
                if target_armor_points < dice_result {
                    let damages_dealt = player.clone().borrow().hit(unwrapped_selected_item.clone(), selected_creature.clone());
                    stdout.write_line(format!("{} inflict {} to {}", player.clone().borrow().name, damages_dealt, selected_creature.clone().borrow().name).as_str())?;
                } else {
                    stdout.write_line(format!("{} cannot inflict damage to {}",
                                              player.clone().borrow().name,
                                              selected_creature.clone().borrow().name).as_str())?;
                }
            }
        };


        return Ok(());
    }

    fn select_target(creatures: &Vec<Rc<RefCell<Pawn>>>, attackable_things: Vec<String>) -> std::io::Result<Rc<RefCell<Pawn>>> {
        let target = menu!(attackable_things);

        let selected_creature = {
            if let Some(t) = target {
                creatures.get(t).unwrap().clone()
            } else {
                //Select default creature
                creatures.get(0).unwrap().clone()
            }
        };
        Ok(selected_creature)
    }
}