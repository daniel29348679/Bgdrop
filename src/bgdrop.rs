use crossbeam::channel::{Sender, unbounded};
use std::thread;

#[derive(Clone)]
pub struct Bgdrop {
    sender: Sender<Box<dyn Send>>,
}

// use dyn has to send everything that is sent to the background thread
// not using dyn would require a specific type, which is not flexible and won't improve performance very much

impl Bgdrop {
    /// Creates a new `Bgdrop` instance with a single background thread.
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<Box<dyn Send>>();

        thread::spawn(move || {
            for _value in receiver {
                // Dropped here
            }
        });

        Bgdrop { sender }
    }

    /// Creates a new `Bgdrop` instance with the specified number of background threads.
    pub fn with_threads(num_threads: usize) -> Self {
        let (sender, receiver) = unbounded::<Box<dyn Send>>();

        for _ in 0..num_threads {
            let receiver = receiver.clone();
            thread::spawn(move || {
                for _value in receiver {
                    // Dropped here
                }
            });
        }

        Bgdrop { sender }
    }
    /// Drops a value in the background thread.
    /// The value must implement `Send` and have a `'static` lifetime.
    /// # Example
    /// ```rust
    /// use bgdrop::Bgdrop;
    /// let bgdrop = Bgdrop::new();
    /// bgdrop.drop(42);
    /// // You can also drop more complex types
    /// ```
    pub fn drop<T: Send + 'static>(&self, value: T) {
        let _ = self.sender.send(Box::new(value));
    }
}
