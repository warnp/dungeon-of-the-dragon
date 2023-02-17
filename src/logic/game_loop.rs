use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::environment::world::World;
use crate::gui::graphical::sprite::{Layer, ObjectToSprite, Sprite};
use crate::gui::menu::Menu;
use crate::interact::actions::Actions;
use crate::pawn::pawn::{Characteristics, Pawn, Position};
use crate::services::initializer::Initializer;
use crate::services::messaging::{MessageContent, Messaging};

pub struct GameLoop {
    menu: Menu,
    messenger: Arc<Mutex<Messaging>>,
}

impl GameLoop {
    pub fn init(messenger: Arc<Mutex<Messaging>>) -> Self {
        Self {
            menu: Menu::init(messenger.clone()),
            messenger,
        }
    }

    pub fn iterate(game_loop: Arc<GameLoop>) {
        thread::spawn(move || {
            let spells = Initializer::generate_spells();

            let mut items = Initializer::generate_items();

            let player1 = Rc::new(RefCell::new(Pawn {
                id: idgenerator::IdInstance::next_id(),
                name: "Toto".to_string(),
                life: 100,
                spell: vec![spells.get(0).unwrap().clone()],
                race: "human".to_string(),
                inventory: vec![Rc::new(items.remove(0))],
                mana: 100,
                characteristics: Characteristics {
                    force: 3,
                    dexterity: 3,
                    constitution: 0,
                    intelligence: 3,
                    willpower: 0,
                    charisma: 0,
                },
                playable: true,
                equipped: Default::default(),
                ai: Rc::new(RefCell::new(None)),
                position: Position { x: 0, y: 0 },
            }));

            let weather_list = Initializer::init_weather();
            let world = Initializer::init(&weather_list, player1.clone(), &mut items);

            //Travel threw places
            GameLoop::loop_handler(world, game_loop.clone()).unwrap();
        });
    }

    fn loop_handler(world: World, game_loop: Arc<GameLoop>) -> std::io::Result<()> {
        loop {
            let world_current_place = world.places.get(0).unwrap();
            game_loop.menu.write_line(format!("You arrived in {}", world_current_place.name).as_str()).unwrap();

            let pawns: &Vec<Rc<RefCell<Pawn>>> = &world_current_place.pawns;

            let pawns_sprites = [pawns.iter().map(|p| {
                p.clone().borrow().get_world_origin()
            })
                .flatten()
                .collect::<Vec<Sprite>>(),
                world_current_place.room.iter()
                    .enumerate()
                    .map(|(row, cols)| cols.iter()
                        .enumerate()
                        .map(|(col, &el)| Sprite::new(el, col as i32, row as i32, Layer::BACKGROUND))
                        .collect::<Vec<Sprite>>())
                    .flatten()
                    .collect::<Vec<Sprite>>()]
                .concat();


            let message_content = MessageContent {
                topic: "sprite".to_string(),
                content: bincode::serialize(&pawns_sprites).unwrap(),
            };

            {
                let messenger_clone = game_loop.clone().messenger.clone();
                let guard = {
                    loop {
                        if let Ok(messenger) = messenger_clone.try_lock() {
                            break messenger;
                        }
                    }
                };


                let (sprite_subscription_sender, _) = guard.get_subscription("sprite").unwrap();
                sprite_subscription_sender.send(message_content).unwrap();
            }

            loop {
                let creatures = (&pawns)
                    .iter()
                    .filter(|e| !e.borrow().playable)
                    .map(|c| {
                        c.borrow().name.clone()
                    })
                    .collect::<Vec<String>>();
                let creatures_count = (&creatures).len();
                game_loop.menu.write_line(format!("There is {} creatures here : {}",
                                                  creatures_count,
                                                  creatures.join(", ")
                ).as_str())?;

                Actions::handle_actions(&Self::order_pawns(pawns)?, &game_loop.clone().menu)?;

                game_loop.menu.clear_line()?;

                let sprites = pawns.iter()
                    .map(|p: &Rc<RefCell<Pawn>>| {
                        Sprite::new(0, p.clone().borrow().position.x as i32, p.clone().borrow().position.y as i32, Layer::MOVABLES)
                    })
                    .collect::<Vec<Sprite>>();

                let message_content = MessageContent {
                    topic: "sprite".to_string(),
                    content: bincode::serialize(&sprites).unwrap(),
                };

                {
                    let messenger_clone = game_loop.clone().messenger.clone();
                    let guard1 = messenger_clone.lock().unwrap();
                    let (sprite_sender, _) = guard1.get_subscription("sprite").unwrap();
                    sprite_sender.send(message_content).unwrap();
                }
            }
        }
    }

    fn order_pawns(pawns: &Vec<Rc<RefCell<Pawn>>>) -> std::io::Result<Vec<Rc<RefCell<Pawn>>>> {
        let mut x = pawns.clone();
        x.sort_by(|a, b| a.borrow().characteristics.dexterity.cmp(&b.borrow().characteristics.dexterity));
        Ok(x)
    }
}