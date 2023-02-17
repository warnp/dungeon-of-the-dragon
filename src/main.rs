use std::env;
use std::sync::{Arc, mpsc, Mutex};
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;
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

    messaging.add_subscription("sprite".to_string());
    messaging.add_subscription("select".to_string());
    messaging.add_subscription("stdout".to_string());


    let messaging_thread = Arc::new(Mutex::new(messaging));
    Messaging::start_bus(messaging_thread.clone()).unwrap();


    let game_loop = GameLoop::init(messaging_thread.clone());
    GameLoop::iterate(Arc::new(game_loop));
    #[cfg(feature = "graphical_mode")]
    let toto = window::init(messaging_thread.clone());
}
