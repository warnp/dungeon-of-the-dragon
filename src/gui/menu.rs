use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;
use crate::services::messaging::MessageContent;

pub struct Menu {
    STDOUT: Term,
    select_menu: Sender<MessageContent>,
    selected_option: Receiver<MessageContent>,
    stdout: Sender<MessageContent>,
    clear: Sender<MessageContent>,
}

impl Menu {
    pub fn init(select_menu: Sender<MessageContent>,
                selected_option: Receiver<MessageContent>,
                stdout: Sender<MessageContent>,
                clear: Sender<MessageContent>,
    ) -> Self {
        Self {
            STDOUT: Term::stdout(),
            selected_option,
            select_menu,
            stdout,
            clear,
        }
    }

    pub fn menu(&self, options: Vec<String>) -> std::io::Result<Option<usize>> {
        #[cfg(not(feature = "graphical_mode"))]
        {
            return Select::with_theme(&ColorfulTheme::default())
                .items(options.as_slice())
                .default(0)
                .interact_on_opt(&Term::stderr());
        }
        #[cfg(feature = "graphical_mode")]
        {
            let vec = options.join(":");
            println!("vec : {}", vec);
            self.select_menu.send(MessageContent {
                topic: "select".to_string(),
                content: vec.as_bytes().to_vec(),
            }).unwrap();


            loop {
                if let Ok(command) = self.selected_option.try_recv() {
                    let ok = Ok(Some(bincode::deserialize(command.content.as_slice()).unwrap()));
                    break ok;
                }
            }
        }
    }

    pub fn write_line(&self, out: &str) -> std::io::Result<()> {
        #[cfg(not(feature = "graphical_mode"))]
        {
            return self.STDOUT.write_line(out)?;
        }

        #[cfg(feature = "graphical_mode")]
        {
            let stdout_topic = "stdout";

            self.stdout.send(MessageContent {
                topic: stdout_topic.to_string(),
                content: out.as_bytes().to_vec(),
            }).unwrap();
        }
        Ok(())
    }

    pub fn clear_line(&self) -> std::io::Result<()> {
        #[cfg(not(feature = "graphical_mode"))]
        self.STDOUT.clear_line()?;

        #[cfg(feature = "graphical_mode")]
        {
            self.clear.send(MessageContent {
                topic: "clear".to_string(),
                content: Vec::new(),
            }).unwrap();
        }


        Ok(())
    }
}