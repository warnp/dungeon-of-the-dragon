use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::environment::world::World;
use crate::gui::graphical::sprite::{Layer, ObjectToSprite, Sprite};
use crate::gui::menu::Menu;
use crate::interact::actions::Actions;
use crate::pawn::pawn::{Characteristics, Pawn, Position};
use crate::services::initializer::Initializer;
use crate::services::messaging::MessageContent;

pub struct GameLoop {}

impl GameLoop {
    pub fn iterate(senders: HashMap<String, Sender<MessageContent>>, mut receivers: HashMap<String, Receiver<MessageContent>>, menu: Menu) {
        thread::spawn(move || {
            let menu = menu;
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

            let world_clone = Arc::new(&world);
            let toto = world_clone.clone();
            let (_,x) = receivers.remove_entry("info").unwrap();
            thread::spawn(move || {
                loop {
                    if let Ok(info) = x.try_recv() {
                        let x1: (f32,f32) = bincode::deserialize(info.content.as_slice()).unwrap();
                        println!("info {:?}", x1);
                        let rooms = toto.places.get(0).unwrap().room;

                    }

                }

            });


            //Travel threw places
            GameLoop::loop_handler(&world, senders, &menu).unwrap();
        });
    }

    fn loop_handler(world: &World, senders: HashMap<String, Sender<MessageContent>>, menu: &Menu) -> std::io::Result<()> {
        let senders = senders;
        loop {
            let world_current_place = world.places.get(0).unwrap();
            menu.write_line(format!("You arrived in {}", world_current_place.name).as_str()).unwrap();

            let pawns: &Vec<Rc<RefCell<Pawn>>> = &world_current_place.pawns;

            let room_tiles = world_current_place.room.iter()
                .enumerate()
                .map(|(row, cols)| cols.iter()
                    .enumerate()
                    .map(|(col, &el)| Sprite::new(el, col as i32, row as i32, Layer::BACKGROUND))
                    .collect::<Vec<Sprite>>())
                .flatten()
                .collect::<Vec<Sprite>>();

            let pawns_sprites = [pawns.iter().map(|p| {
                p.clone().borrow().get_world_origin()
            })
                .flatten()
                .collect::<Vec<Sprite>>(),
                room_tiles.clone()]
                .concat();


            let message_content = MessageContent {
                topic: "sprite".to_string(),
                content: bincode::serialize(&pawns_sprites).unwrap(),
            };

            let sender = senders.get("sprite").unwrap();
            sender.send(message_content).unwrap();


            loop {
                let creatures = (&pawns)
                    .iter()
                    .filter(|e| !e.borrow().playable)
                    .map(|c| {
                        c.borrow().name.clone()
                    })
                    .collect::<Vec<String>>();
                let creatures_count = (&creatures).len();
                menu.write_line(format!("There is {} creatures here : {}",
                                        creatures_count,
                                        creatures.join(", ")
                ).as_str())?;

                Actions::handle_actions(&Self::order_pawns(pawns)?, menu)?;

                menu.clear_line()?;

                let sprites = [pawns.iter()
                    .map(|p: &Rc<RefCell<Pawn>>| p.clone().borrow().get_world_origin())
                    .flatten()
                    .collect::<Vec<Sprite>>(),
                    room_tiles.clone()
                ].concat();


                let message_content = MessageContent {
                    topic: "sprite".to_string(),
                    content: bincode::serialize(&sprites).unwrap(),
                };

                let sender = senders.get("sprite").unwrap();
                sender.send(message_content).unwrap();
            }
        }
    }

    fn order_pawns(pawns: &Vec<Rc<RefCell<Pawn>>>) -> std::io::Result<Vec<Rc<RefCell<Pawn>>>> {
        let mut x = pawns.clone();
        x.sort_by(|a, b| a.borrow().characteristics.dexterity.cmp(&b.borrow().characteristics.dexterity));
        Ok(x)
    }
}