use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::logic::game_loop::GameLoop;
use crate::gui::graphical::window;
use crate::gui::menu::Menu;
use crate::services::messaging::{MessageContent, Messaging};

mod pawn;
mod inventory;
mod environment;
mod services;
mod interact;
mod logic;
mod gui;
mod ai;


fn main() {
    let mut messaging = Messaging::init();

    let select = messaging.create_topic();
    let select_response = messaging.subscribe_to_topic("select_response".to_string());
    let stdout = messaging.create_topic();
    let clear = messaging.create_topic();
    let menu = {
        Menu::init(select,
                   select_response,
                   stdout,
        clear)
    };

    let mut messenger_gameplay_sender = HashMap::new();
    let mut messenger_gameplay_receiver = HashMap::new();
    messenger_gameplay_sender.insert("sprite".to_string(), messaging.create_topic());
    messenger_gameplay_sender.insert("info_response".to_string(), messaging.create_topic());
    messenger_gameplay_sender.insert("gameplay_state".to_string(), messaging.create_topic());
    messenger_gameplay_receiver.insert("info".to_string(), messaging.subscribe_to_topic("info".to_string()));

    let mut messenger_ui_map_receiver = HashMap::new();
    let mut messenger_ui_map_sender = HashMap::new();

    messenger_ui_map_receiver.insert("sprite".to_string(), messaging.subscribe_to_topic("sprite".to_string()));
    messenger_ui_map_receiver.insert("stdout".to_string(), messaging.subscribe_to_topic("stdout".to_string()));
    messenger_ui_map_receiver.insert("select".to_string(), messaging.subscribe_to_topic("select".to_string()));
    messenger_ui_map_receiver.insert("clear".to_string(), messaging.subscribe_to_topic("clear".to_string()));
    messenger_ui_map_receiver.insert("info_response".to_string(), messaging.subscribe_to_topic("info_response".to_string()));
    messenger_ui_map_receiver.insert("gameplay_state".to_string(), messaging.subscribe_to_topic("gameplay_state".to_string()));
    messenger_ui_map_sender.insert("select_response".to_string(), messaging.create_topic());
    messenger_ui_map_sender.insert("info".to_string(), messaging.create_topic());


    Messaging::start_bus(messaging.incoming_messages, messaging.outcoming_messages).unwrap();

    GameLoop::iterate(messenger_gameplay_sender, messenger_gameplay_receiver, menu);


    // #[cfg(feature = "graphical_mode")]
    window::init(messenger_ui_map_receiver, messenger_ui_map_sender).unwrap();
}
