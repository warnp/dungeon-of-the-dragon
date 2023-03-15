use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

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
        println!("Bus is starting");
        println!("Open topics are {:#?}", outcoming_messages.iter()
            .map(|(topic, _)| topic.clone())
            .collect::<Vec<String>>());

        let handle = thread::spawn(move || {
            println!("Starting message bus loop");

            //We handle message while we do not get lifecycle message to close the bus
            loop {
                incoming_messages.iter()
                    .for_each(|rx| {
                        if let Ok(message_content) = rx.try_recv() {
                            // println!("Broadcasting on {} topic", message_content.topic);
                            outcoming_messages.iter()
                                .enumerate()
                                .filter(|(j, (topic, _))|
                                   message_content.topic.clone() == topic.clone()
                                )
                                .for_each(|(_, (topic,sender))|{
                                    // println!("Topic to send {}", topic);
                                    match sender.send(message_content.clone()) {
                                        Ok(()) => {
                                            // println!("Message on topic {} distibuted", topic)
                                        },
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