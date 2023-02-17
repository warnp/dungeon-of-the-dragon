use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Default)]
pub struct MessageContent {
    pub topic: String,
    pub content: Vec<u8>,
}

pub struct Messaging {
    incoming_messages: Vec<Receiver<MessageContent>>,
    outcoming_messages: HashMap<String, Sender<MessageContent>>,
    bus_thread: Option<JoinHandle<()>>,
    thread_lifecycle: Option<Sender<bool>>,
    channels: HashMap<String, (Sender<MessageContent>, Receiver<MessageContent>)>
}

impl Messaging {
    pub fn init() -> Self {
        Self {
            incoming_messages: vec![],
            outcoming_messages: HashMap::new(),
            bus_thread: None,
            thread_lifecycle: None,
            channels: HashMap::new(),
        }
    }

    pub fn add_subscription(&mut self, topic: String) {
        let (channel_client_sender, bus_receiver) = mpsc::channel();
        let (bus_sender, channel_client_receiver) = mpsc::channel();

        self.incoming_messages.push(bus_receiver);
        self.outcoming_messages.insert(topic.clone(), bus_sender);

        self.channels.insert(topic, (channel_client_sender, channel_client_receiver));
    }

    pub fn get_subscription(&self, topic: &str) -> Option<&(Sender<MessageContent>, Receiver<MessageContent>)> {
        self.channels.get(topic)
    }

    pub fn start_bus(messaging: Arc<Mutex<Messaging>>) -> thread::Result<()> {
        let messaging_cloned = messaging.clone();
        if let Some(running_tread) = &messaging_cloned.lock().unwrap().bus_thread {
            if let Some(lifecycle) = &messaging_cloned.lock().unwrap().thread_lifecycle {
                lifecycle.send(true).unwrap();
            }
        }

        let (lifecycle_tx, lifecycle_rx) = mpsc::channel();
        {
            let arc = messaging_cloned.clone();
            let mut guard = arc.lock().unwrap();
            guard.thread_lifecycle = Some(lifecycle_tx);
        }


        let handle = thread::spawn(move || {
            let mut lifecycle_result = false;
            println!("Starting message bus loop");

            //We handle message while we do not get lifecycle message to close the bus
            while !lifecycle_result {

                lifecycle_result = {
                    if let Ok(lifecycle) = lifecycle_rx.try_recv() {
                        lifecycle
                    } else {
                        false
                    }
                };

                let mut bus_content: HashMap<String, MessageContent> = HashMap::new();

                let messaging_borrow = messaging_cloned.clone();

                let guard = {
                    loop {
                        if let Ok(messenger) = messaging_borrow.try_lock() {
                            break messenger;
                        }
                    }
                };

                guard.incoming_messages.iter()
                    .for_each(|rx| {
                        if let Ok(recv) = rx.try_recv() {
                            bus_content.insert(recv.topic.clone(), recv);
                        }
                    });

                bus_content.iter()
                    .for_each(|(k, v)| {
                        println!("Message on topic {} distibuted", k);
                        if let Some(sender) = messaging_borrow.lock().unwrap().outcoming_messages.get(k) {
                            sender.send(v.clone()).unwrap();
                        }
                    });
            }
            println!("Stop message bus loop");
        });

        messaging.clone().lock().unwrap().bus_thread = Some(handle);
        Ok(())
    }
}