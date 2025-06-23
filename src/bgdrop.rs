use std::sync::mpsc::{self, Sender};
use std::thread;

/// A handle to the background drop system.
#[derive(Clone)]
pub struct Bgdrop {
    sender: Sender<Box<dyn Send>>,
}

impl Bgdrop {
    /// Create a new Bgdrop handler and start the background thread.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<Box<dyn Send>>();

        thread::spawn(move || {
            for _value in receiver {
                // When `_value` is dropped here, it triggers T's drop in background
                // Nothing else is needed
            }
        });

        Bgdrop { sender }
    }

    pub fn with_threads(num_threads: usize) -> Self {
        use std::sync::{Arc, Mutex};

        let (sender, receiver) = mpsc::channel::<Box<dyn Send>>();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..num_threads {
            let receiver_clone = Arc::clone(&receiver);
            thread::spawn(move || {
                loop {
                    let value = {
                        let lock = receiver_clone.lock().unwrap();
                        lock.recv()
                    };
                    match value {
                        Ok(_value) => {
                            // When `_value` is dropped here, it triggers T's drop in background
                            // Nothing else is needed
                        }
                        Err(_) => break, // Channel closed
                    }
                }
            });
        }

        Bgdrop { sender }
    }

    /// Submit a value to be dropped in the background.
    pub fn drop<T: Send + 'static>(&self, value: T) {
        let _ = self.sender.send(Box::new(value));
    }
}
