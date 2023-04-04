use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::current;
use console::Term;
use rand::random;
use serde::{Deserialize, Serialize};
use crate::inventory::item::{DamageTypeEnum, Item, ItemAttackTypeEnum, PartToEquiEnum, Pocketable};
use crate::ai;
use crate::ai::ai::let_ai_or_human_play;
use crate::pawn::pawn::{Pawn, Position};
use crate::services::dice::RollDiceResult;
use crate::services::interactions::Attack;
use crate::Select;
use crate::ColorfulTheme;
use crate::environment::world::{Place, World};
use crate::gui::menu::Menu;
use crate::services::a_star::calculate_range;
use crate::services::messaging::MessageContent;

#[warn(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Actions {
    OPEN = 0,
    ATTACK,
    WALK_TO,
    WATCH,
    USE,
    EQUIP,
}

impl Actions {
    pub fn vec_string() -> Vec<String> {
        vec!["Open".to_string(),
             "Attack".to_string(),
             "Walk to".to_string(),
             "Watch".to_string(),
             "Use".to_string(),
             "Equip".to_string()]
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

impl Actions {
    pub fn handle_actions(pawns: &Vec<Rc<RefCell<Pawn>>>,
                          world: &World,
                          receivers: &HashMap<String, Receiver<MessageContent>>,
                          senders: &HashMap<String, Sender<MessageContent>>,
                          menu: &Menu) -> std::io::Result<()> {
        let mut pawns_iter = (&pawns).iter();
        while let Some(current_pawn) = pawns_iter.next() {
            menu.write_line(format!("{} turn.", current_pawn.clone().borrow().name).as_str())?;

            senders.get("current_player").unwrap().send(MessageContent {
                topic: "current_player".to_string(),
                content: bincode::serialize(&current_pawn.clone().borrow().id).unwrap(),
            }).unwrap();

            println!("debut de tour de {}", current_pawn.clone().borrow().name);

            let actions = ai::ai::let_ai_or_human_play(current_pawn.clone(),
                                                       || {
                                                           if let Ok(selected_action_id) = menu.menu(Actions::vec_string()) {
                                                               selected_action_id
                                                           } else {
                                                               None
                                                           }
                                                       },
                                                       || {
                                                           if let Some(mut ref_mut) = current_pawn.clone().borrow().ai.clone().borrow_mut().to_owned() {
                                                               let i = ref_mut.get_action(current_pawn.clone(), menu).unwrap();
                                                               return Some(i);
                                                           }
                                                           None
                                                       });

            menu.clear_line()?;

            let graphical_mode = false;
            #[cfg(feature = "graphical_mode")] let graphical_mode = true;

            if let Some(action) = actions {
                match action.into() {
                    Actions::USE => {
                        println!("USE");
                        Ok(())
                    }
                    Actions::WATCH => Self::watch_action(current_pawn.clone(), pawns, world, receivers, senders, menu, graphical_mode),
                    Actions::WALK_TO => Self::walk_action(&world.places.get(0).unwrap().room, receivers, senders, menu, current_pawn, &world.places.iter().map(|el| el.id).collect::<Vec<u8>>()),
                    Actions::ATTACK => Self::attack_action(pawns, current_pawn.clone(), senders, receivers, menu, &world.places.get(0).unwrap().room, graphical_mode),
                    Actions::OPEN => {
                        println!("OPEN");

                        Ok(())
                    }
                    Actions::EQUIP => Self::equip_item(current_pawn.clone(), menu)
                }?;
            }
            println!("fin de tour de {}", current_pawn.clone().borrow().name);

        }

        Ok(())
    }

    fn walk_action(room: &Vec<Vec<u8>>, receivers: &HashMap<String, Receiver<MessageContent>>, senders: &HashMap<String, Sender<MessageContent>>, menu: &Menu, current_pawn: &Rc<RefCell<Pawn>>, places_id: &Vec<u8>) -> Result<(), Error> {
        senders.get("gameplay_state").unwrap().send(MessageContent {
            topic: "gameplay_state".to_string(),
            content: bincode::serialize(&Actions::WALK_TO).unwrap(),
        }).unwrap();

        let stats = (current_pawn.clone().borrow().characteristics.dexterity as u16 + current_pawn.clone().borrow().characteristics.force as u16) / 3u16;

        let range = Self::calculate_range(current_pawn.clone(), room, stats);

        let selected_target = loop {
            let selected_target = Self::communicate_to_ui_for_target(&range, senders, receivers);
            let x = range.get(selected_target.1 as usize).unwrap().get(selected_target.0 as usize).unwrap();
            if *x {
                break selected_target;
            }
        };
        let desired_next_position = room.get(selected_target.1 as usize).unwrap().get(selected_target.0 as usize).unwrap();

        let door_id = places_id.iter()
            .filter(|&&el| {
                el == desired_next_position.clone()
            })
            .collect::<Vec<&u8>>()
            .first();

        let current_pawn_id = current_pawn.clone().borrow().id;
        let current_pawn_name = current_pawn.clone().borrow().name.clone();

        if let Some(&id) = door_id {
            Self::send_end_turn_signal(senders, current_pawn_id);

            menu.write_line(format!("{} walk to the door...", current_pawn_name).as_str())?;
        }else if desired_next_position == &20u8 {
            Self::send_end_turn_signal(senders, current_pawn_id);

            menu.write_line(format!("{} cannot walk there", current_pawn_name).as_str())?;
        }else {

            let current_pawn_clone = current_pawn.clone();
            let mut ref_mut1 = current_pawn_clone.borrow_mut();
            ref_mut1.position = Position { x: selected_target.0, y: selected_target.1 };

            Self::send_end_turn_signal(senders, current_pawn_id);

            menu.write_line(format!("{} walk...", current_pawn_name).as_str())?;
        }
        Ok(())
    }

    fn send_end_turn_signal(senders: &HashMap<String, Sender<MessageContent>>, current_pawn_id: i64) {
        senders.get("end_turn").unwrap().send(MessageContent {
            topic: "end_turn".to_string(),
            content: bincode::serialize(&("end_attack", current_pawn_id)).unwrap(),
        }).unwrap();
    }

    fn watch_action(current_player: Rc<RefCell<Pawn>>,
                    creatures: &Vec<Rc<RefCell<Pawn>>>,
                    world: &World,
                    receivers: &HashMap<String, Receiver<MessageContent>>,
                    senders: &HashMap<String, Sender<MessageContent>>,
                    menu: &Menu,
                    graphical_mode: bool) -> std::io::Result<()> {
        #[cfg(feature = "graphical_mode")]
        if current_player.clone().borrow().playable {
            senders.get("gameplay_state").unwrap().send(MessageContent {
                topic: "gameplay_state".to_string(),
                content: bincode::serialize(&Actions::WATCH).unwrap(),
            }).unwrap();

            loop {
                if let Ok(command) = receivers.get("info").unwrap().try_recv() {
                    let (x, y): (u16, u16) = bincode::deserialize(command.content.as_slice()).unwrap();

                    println!("position {}, {}", x, y);
                    let creatures = creatures.iter()
                        .filter(|c| c.borrow().position.y == y && c.borrow().position.x == x)
                        .map(|el| el.clone())
                        .collect::<Vec<Rc<RefCell<Pawn>>>>();

                    if let Some(creature_watched) = creatures.first() {
                        let creature_watched = creature_watched.clone();
                        let creature_stats = current_player.clone().borrow().try_watch(creature_watched);

                        senders.get("info_response").unwrap().send(MessageContent {
                            topic: "info_response".to_string(),
                            content: creature_stats.as_str().as_bytes().to_vec(),
                        }).unwrap();
                    } else {
                        let place: &Place = world.places.get(0).unwrap();
                        let tile_spec = place.room.get(y as usize)
                            .unwrap()
                            .get(x as usize)
                            .unwrap();

                        let tile_info = match tile_spec {
                            10 => "Simple floor",
                            11 => "Path to First room",
                            12 => "Path to Second room",
                            _ => ""
                        };

                        senders.get("info_response").unwrap().send(MessageContent {
                            topic: "info_response".to_string(),
                            content: tile_info.as_bytes().to_vec(),
                        }).unwrap();
                    }
                    loop {
                        if let Ok(command) = receivers.get("info").unwrap().try_recv() {
                            break;
                        }
                    }
                    break;
                }
            }
        }

        if !graphical_mode || !current_player.clone().borrow().playable {
            if creatures.is_empty() {
                menu.write_line("Nothing to see here")?;
                return Ok(());
            }

            //TODO use perception here
            //TODO Add item in room
            let creatures_name = {
                if current_player.clone().borrow().playable {
                    menu.write_line("Watch what ?")?;
                    let creatures_name = creatures.iter()
                        .filter(|e| e.clone().borrow().life > 0 &&
                            e.clone().borrow().id != current_player.clone().borrow().id)
                        .map(|e| e.clone().borrow().name.clone())
                        .collect::<Vec<String>>();
                    creatures_name
                } else {
                    vec![]
                }
            };

            let selected_creature_index: Option<usize> = ai::ai::let_ai_or_human_play(current_player.clone(), move || {
                if let Ok(result) = menu.menu(creatures_name.clone()) {
                    result
                } else {
                    None
                }
            }, || {
                let creatures_number = creatures.iter()
                    .filter(|e| {
                        let pawn_clone = e.clone();
                        pawn_clone.borrow().life > 0 &&
                            pawn_clone.borrow().playable &&
                            pawn_clone.borrow().id != current_player.clone().borrow().id
                    }).count();
                let random_id = ((rand::random::<f32>() * creatures_number as f32) as f32).floor() as usize;

                Some(random_id)
            });

            if let Some(creature_id) = selected_creature_index {
                if current_player.clone().borrow().playable {
                    let creature = creatures.get(creature_id).unwrap();
                    menu.write_line(format!("{:#?}", creature).as_str())?;
                } else {
                    let creatures = creatures.iter()
                        .filter(|e| {
                            let pawn_clone = e.clone();
                            pawn_clone.borrow().life > 0 &&
                                pawn_clone.borrow().playable &&
                                pawn_clone.borrow().id != current_player.clone().borrow().id
                        })
                        .map(|e| e.clone())
                        .collect::<Vec<Rc<RefCell<Pawn>>>>();
                    let creature = creatures
                        .get(creature_id)
                        .unwrap();

                    let clonned_current_player = current_player.clone();
                    menu.write_line(format!("{} is watching...", clonned_current_player.borrow().name).as_str())?;
                    let ref_mut = clonned_current_player.borrow_mut();
                    let option = ref_mut.ai.clone();
                    let mut ai = option.borrow_mut();
                    ai.as_mut().unwrap().add_target_to_watched_target(creature.clone());
                }
            }
        }

        Ok(())
    }


    fn equip_item(player: Rc<RefCell<Pawn>>, menu: &Menu) -> std::io::Result<()> {
        let selected_item: Option<Rc<Item>> =
            {
                let borrowed_player = player.borrow();

                let mut inventory = borrowed_player
                    .inventory
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<&str>>();

                inventory.push("Unequip");

                let items = menu.menu(inventory.iter().map(|el| el.to_string()).collect::<Vec<String>>()).unwrap();

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
            menu.write_line(format!("You equipped {}", item.clone().name).as_str())?;
            mutable_player.equip(Rc::clone(&item));
        } else {
            menu.write_line("What part do you want to unequip?")?;
            let part_to_unequip = menu.menu([
                PartToEquiEnum::HEAD,
                PartToEquiEnum::FEET,
                PartToEquiEnum::LEGS,
                PartToEquiEnum::RIGHT_HAND,
                PartToEquiEnum::LEFT_HAND,
                PartToEquiEnum::BODY
            ].iter().map(|el| el.to_string()).collect::<Vec<String>>());

            let equip_enum = {
                if let Some(part) = part_to_unequip.unwrap() {
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

    fn attack_action(creatures: &Vec<Rc<RefCell<Pawn>>>,
                     player: Rc<RefCell<Pawn>>,
                     senders: &HashMap<String, Sender<MessageContent>>,
                     receivers: &HashMap<String, Receiver<MessageContent>>,
                     menu: &Menu,
                     room: &Vec<Vec<u8>>,
                     graphical_mode: bool) -> std::io::Result<()> {
        senders.get("gameplay_state").unwrap().send(MessageContent {
            topic: "gameplay_state".to_string(),
            content: bincode::serialize(&Actions::ATTACK).unwrap(),
        }).unwrap();
        println!("current player {}", player.clone().borrow().name);

        if player.borrow().playable {
            menu.write_line("with ?")?;
        }

        //Select action
        let action = ai::ai::let_ai_or_human_play(player.clone(),
                                                  || menu.menu(vec!["your equipped weapon".to_string(), "a spell".to_string()]).unwrap(),
                                                  || {
                                                      let rc = player.clone();
                                                      let x = rc.borrow();
                                                      let rc1 = x.ai.clone();
                                                      let ref_mut = rc1.borrow_mut();
                                                      if let Some(ai) = ref_mut.to_owned() {
                                                          return Some(ai.select_weapon_or_spell(player.clone()));
                                                      }
                                                      None
                                                  });

        let select_item_to_attack_with = Attack::select_item_to_attack_with(player.clone(), action, menu)?;

        if let None = select_item_to_attack_with {
            menu.write_line("You have no way to deal damage to any target!")?;
            return Ok(());
        }
        let unwrapped_selected_item = select_item_to_attack_with.unwrap();

        if let None = unwrapped_selected_item.get_range() {
            menu.write_line("You have no way to deal damage to any target!")?;
            return Ok(());
        }

        let range = Self::calculate_range(player.clone(), room, unwrapped_selected_item.get_range().unwrap());

        let attackable_things = creatures.clone()
            .iter()
            .filter(|&e| {
                e.borrow().life > 0 &&
                    range.get(e.borrow().position.y as usize)
                        .unwrap()
                        .get(e.borrow().position.x as usize)
                        .unwrap().clone()
            })
            .map(|c| {
                c.borrow().name.clone()
            })
            .collect::<Vec<String>>();

        //TODO Add attackable element if any

        if attackable_things.is_empty() {
            menu.write_line("There is nothing to attack")?;
            return Ok(());
        }

        let usability = player.borrow().calculate_usability(unwrapped_selected_item.clone(), menu)?;

        if usability == 0 {
            menu.write_line("You don't know what to do!")?;
            return Ok(());
        }

        menu.write_line("Attack what?")?;
        let playable = player.clone().borrow().playable;

        let selected_creature = if !graphical_mode || !playable {
            let targeted_creature = Self::select_target_console(creatures, attackable_things, player.clone(), menu)?;
            if graphical_mode && !playable {
                let toto: Vec<Vec<bool>> = Vec::new();
                senders.get("targetable").unwrap().send(MessageContent {
                    topic: "targetable".to_string(),
                    content: bincode::serialize(&toto).unwrap(),
                }).unwrap();

                let targeted_creature = targeted_creature.clone();

                let position = (targeted_creature.borrow().position.x, targeted_creature.borrow().position.y);
                Self::send_damage_type_message(senders, &unwrapped_selected_item.get_damage_type().unwrap(), &position);
            }
            targeted_creature
        } else {

            Self::select_target_ui(range, player.clone(), creatures, senders, receivers, &unwrapped_selected_item.get_damage_type().unwrap(), menu)?
        };
        menu.write_line("Roll 1d20 : ")?;

        Self::roll_dice_attack(player.clone(), unwrapped_selected_item, selected_creature, menu)?;

        if graphical_mode {
            senders.get("end_turn").unwrap().send(MessageContent {
                topic: "end_turn".to_string(),
                content: bincode::serialize(&("end_attack".as_bytes().to_vec(), player.clone().borrow().id)).unwrap(),
            }).unwrap();
        }

        return Ok(());
    }

    fn roll_dice_attack(player: Rc<RefCell<Pawn>>, unwrapped_selected_item: Rc<dyn Pocketable>, selected_creature: Rc<RefCell<Pawn>>, menu: &Menu) -> std::io::Result<()> {
        // Roll dice
        match Attack::roll_attack() {
            RollDiceResult::Critical => Self::crititcal(&player, &unwrapped_selected_item, selected_creature, menu)?,
            RollDiceResult::Fumble => Self::fumble(menu)?,
            RollDiceResult::Normal(dice_result) => Self::normal(player, unwrapped_selected_item, selected_creature, dice_result, menu)?,
        };

        Ok(())
    }

    fn normal(player: Rc<RefCell<Pawn>>, unwrapped_selected_item: Rc<dyn Pocketable>, selected_creature: Rc<RefCell<Pawn>>, dice_result: u8, menu: &Menu) -> std::io::Result<()> {
        menu.write_line(format!("Normal attack, dice result : {}", dice_result).as_str())?;

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
            let player_clone = player.clone();
            let damages_dealt = player_clone.borrow().hit(unwrapped_selected_item.clone(), selected_creature.clone());
            menu.write_line(format!("{} inflict {} to {}", player_clone.borrow().name, damages_dealt, selected_creature.clone().borrow().name).as_str())?;
        } else {
            menu.write_line(format!("{} cannot inflict damage to {}",
                                    player.clone().borrow().name,
                                    selected_creature.clone().borrow().name).as_str())?;
        }

        Ok(())
    }

    fn fumble(menu: &Menu) -> std::io::Result<()> {
        menu.write_line("Fumble")?;
        Ok(())
    }

    fn crititcal(player: &Rc<RefCell<Pawn>>, unwrapped_selected_item: &Rc<dyn Pocketable>, selected_creature: Rc<RefCell<Pawn>>, menu: &Menu) -> std::io::Result<()> {
        menu.write_line("Critical!")?;

        let damages_dealt = player.clone().borrow().hit(unwrapped_selected_item.clone(), selected_creature.clone());
        menu.write_line(format!("{} inflict {} to {}", player.clone().borrow().name, damages_dealt, selected_creature.clone().borrow().name).as_str())?;
        Ok(())
    }

    fn select_target_console(creatures: &Vec<Rc<RefCell<Pawn>>>, attackable_things: Vec<String>, player: Rc<RefCell<Pawn>>, menu: &Menu) -> std::io::Result<Rc<RefCell<Pawn>>> {
        let target = {
            if player.clone().borrow().playable {
                menu.menu(attackable_things).unwrap()
            } else {
                let playable_pawns = creatures.iter()
                    .filter(|c| {
                        let creature_clone = c.clone();
                        creature_clone.borrow().playable &&
                            creature_clone.borrow().id != player.clone().borrow().id
                    })
                    .collect::<Vec<&Rc<RefCell<Pawn>>>>();
                let i = creatures.iter()
                    .position(|c| playable_pawns.get((random::<f32>() * playable_pawns.len() as f32).floor() as usize)
                        .unwrap()
                        .clone()
                        .borrow()
                        .id == c.clone().borrow().id)
                    .unwrap();

                Some(i)
            }
        };
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

    fn select_target_ui(range: Vec<Vec<bool>>,
                        player: Rc<RefCell<Pawn>>,
                        creatures: &Vec<Rc<RefCell<Pawn>>>,
                        senders: &HashMap<String, Sender<MessageContent>>,
                        receivers: &HashMap<String, Receiver<MessageContent>>,
                        damage_type: &DamageTypeEnum,
                        menu: &Menu) -> std::io::Result<Rc<RefCell<Pawn>>> {
        loop {
            let selected_target = Self::communicate_to_ui_for_target(&range, senders, receivers);

            let filtered_creatures = creatures.iter()
                .filter(|el| {
                    Position {
                        x: selected_target.0,
                        y: selected_target.1,
                    } == el.clone().borrow().position &&
                        el.clone().borrow().id != player.clone().borrow().id
                })
                .map(|el| el.clone())
                .collect::<Vec<Rc<RefCell<Pawn>>>>();


            if !filtered_creatures.is_empty() {
                let targeted_creature = filtered_creatures.first().unwrap().clone();

                let position = (targeted_creature.borrow().position.x, targeted_creature.borrow().position.y);
                Self::send_damage_type_message(senders, damage_type, &position);
                return Ok((creatures.get(0).unwrap().clone()));
            } else {
                menu.write_line("No target selected. Try again.")?;
            }
        }
    }

    fn communicate_to_ui_for_target(range: &Vec<Vec<bool>>, senders: &HashMap<String, Sender<MessageContent>>, receivers: &HashMap<String, Receiver<MessageContent>>) -> (u16, u16) {
        senders.get("targetable").unwrap().send(MessageContent {
            topic: "targetable".to_string(),
            content: bincode::serialize(&range).unwrap(),
        }).unwrap();

        let info_receiver = receivers.get("info").unwrap();
        let selected_target: (u16, u16) = loop {
            if let Ok(info) = info_receiver.try_recv() {
                break bincode::deserialize(info.content.as_slice()).unwrap();
            }
        };
        selected_target
    }

    fn send_damage_type_message(senders: &HashMap<String, Sender<MessageContent>>, damage_type: &DamageTypeEnum, position: &(u16, u16)) {
        senders.get("show_damage").unwrap().send(MessageContent {
            topic: "show_damage".to_string(),
            content: bincode::serialize(&(position, damage_type)).unwrap(),
        }).unwrap();
    }

    fn calculate_range(player: Rc<RefCell<Pawn>>, room: &Vec<Vec<u8>>, range: u16) -> Vec<Vec<bool>> {
        let (x, y) = (player.clone().borrow().position.x, player.clone().borrow().position.y);
        let range = calculate_range((x, y), range, room);
        range
    }
}