use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Default, Debug)]
pub struct MessageContent {
    pub topic: String,
    pub content: Vec<u8>,
}

pub struct Messaging {
    pub incoming_messages: Vec<Receiver<MessageContent>>,
    pub outcoming_messages: Vec<(String, Sender<MessageContent>)>,
}

impl Messaging {
    pub fn init() -> Self {
        Self {
            incoming_messages: vec![],
            outcoming_messages: vec![],
        }
    }

    pub fn create_topic(&mut self) -> Sender<MessageContent> {
        let (channel_client_sender, bus_receiver) = mpsc::channel();
        self.incoming_messages.push(bus_receiver);
        channel_client_sender
    }

    pub fn subscribe_to_topic(&mut self, topic: String) -> Receiver<MessageContent> {
        let (bus_sender, channel_client_receiver) = mpsc::channel();
        self.outcoming_messages.push((topic.clone(), bus_sender));
        channel_client_receiver
    }


    pub fn start_bus(incoming_messages: Vec<Receiver<MessageContent>>, outcoming_messages: Vec<(String, Sender<MessageContent>)>) -> thread::Result<()> {
        let handle = thread::spawn(move || {
            println!("Starting message bus loop");

            //We handle message while we do not get lifecycle message to close the bus
            loop {
                incoming_messages.iter()
                    .enumerate()
                    .for_each(|(i, rx)| {
                        if let Ok(message_content) = rx.try_recv() {
                            outcoming_messages.iter()
                                .enumerate()
                                .filter(|(j, (topic, _))|
                                   i != j.clone() && message_content.topic.clone() == topic.clone()
                                )
                                .for_each(|(_, (topic,sender))|{
                                    match sender.send(message_content.clone()) {
                                        Ok(()) => println!("Message on topic {} distibuted", topic),
                                        Err(e) => {
                                            println!("Error while distributing message on topic {} : {:#?}", topic, e.to_string());
                                        }
                                    }
                                })
                        }
                    });
            }
        });

        Ok(())
    }
}