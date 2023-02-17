use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;
use crate::services::messaging::{MessageContent, Messaging};

lazy_static! {
    static ref STDOUT: Term = Term::stdout();
}

pub struct Menu {
    messenger: Arc<Mutex<Messaging>>,
}

impl Menu {
    pub fn init(messenger: Arc<Mutex<Messaging>>) -> Self {
        Self {
            messenger
        }
    }

    pub fn menu(&self, options: Vec<String>) -> std::io::Result<Option<usize>> {
        #[cfg(not(feature = "graphical_mode"))]
        {
            return Select::with_theme(&ColorfulTheme::default())
                .items(options.as_slice())
                .default(0)
                .interact_on_opt(&Term::stderr())?;
        }
        #[cfg(feature = "graphical_mode")]
        {
            let mut x = self.messenger.lock().unwrap();
            let (tx, rx) = x.get_subscription("select").unwrap();

            tx.send(MessageContent {
                topic: "select".to_string(),
                content: bincode::serialize(&options.iter().map(|el| el.clone()).collect::<Vec<String>>()).unwrap(),
            }).unwrap();

            std::mem::drop(x);

            let response_topic = "select_response";
            let guard = self.messenger.lock().unwrap();
            let (tx, rx) = guard.get_subscription(response_topic).unwrap();
            loop {
                if let Ok(command) = rx.try_recv() {
                    if response_topic == command.topic.as_str() {
                        return Ok(Some(bincode::deserialize(command.content.as_slice()).unwrap()));
                    } else {
                        return Ok(Some(0));
                    }
                }
            }
            std::mem::drop(guard);

        }
    }

    pub fn write_line(&self, out: &str) -> std::io::Result<()> {
        #[cfg(not(feature = "graphical_mode"))]
        {
            return STDOUT::write_line(out)?;
        }

        #[cfg(feature = "graphical_mode")]
        {
            let mut guard = {
                loop {
                    if let Ok(messenger) = self.messenger.try_lock() {
                        break messenger;
                    }
                }
            };
            let stdout_topic = "stdout";
            let (tx, _) = guard.get_subscription(stdout_topic).unwrap();
            tx.send(MessageContent {
                topic: stdout_topic.to_string(),
                content: bincode::serialize(out).unwrap(),
            }).unwrap();
            drop(guard);
        }
        Ok(())
    }

    pub fn clear_line(&self) -> std::io::Result<()>{
        #[cfg(not(feature = "graphical_mode"))]
        STDOUT.clear_line()?;
        Ok(())
    }
}