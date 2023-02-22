use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;
use crate::services::messaging::MessageContent;

lazy_static! {
    static ref STDOUT: Term = Term::stdout();
}

pub struct Menu {
    select_menu: Sender<MessageContent>,
    selected_option: Receiver<MessageContent>,
    stdout: Sender<MessageContent>,
}

impl Menu {
    pub fn init(select_menu: Sender<MessageContent>, selected_option: Receiver<MessageContent>, stdout: Sender<MessageContent>) -> Self {
        Self {
            selected_option,
            select_menu,
            stdout
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
            self.select_menu.send(MessageContent {
                topic: "select".to_string(),
                content: bincode::serialize(&options.iter().map(|el| format!("{}\n",el.clone())).collect::<Vec<String>>()).unwrap(),
            }).unwrap();


            loop {
                thread::sleep(Duration::new(0, 1000));
                // println!("pouet {:#?}", self.selected_option);
                if let Ok(command) = self.selected_option.try_recv() {
                    println!("menu result {:#?}", command);
                    if "select_response" == command.topic.as_str() {
                        let ok = Ok(Some(bincode::deserialize(command.content.as_slice()).unwrap()));
                        println!("menu result {:#?}", ok);
                        return ok;
                    }
                }
            }

        }
    }

    pub fn write_line(&self, out: &str) -> std::io::Result<()> {
        #[cfg(not(feature = "graphical_mode"))]
        {
            return STDOUT::write_line(out)?;
        }

        #[cfg(feature = "graphical_mode")]
        {
            let stdout_topic = "stdout";

            self.stdout.send(MessageContent {
                topic: stdout_topic.to_string(),
                content: bincode::serialize(out).unwrap(),
            }).unwrap();
        }
        Ok(())
    }

    pub fn clear_line(&self) -> std::io::Result<()>{
        #[cfg(not(feature = "graphical_mode"))]
        STDOUT.clear_line()?;
        Ok(())
    }
}