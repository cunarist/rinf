use std::sync::mpsc;
use std::thread;

// Create a thread-local sender that lives at the main thread.
thread_local! {
    pub static SENDER: mpsc::Sender<Box<dyn FnOnce() + Send>> = {
        let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
        // Spawn a new thread for the receiver to process bridge tasks.
        thread::spawn(move || {
            while let Ok(bridge_task) = receiver.recv() {
                bridge_task();
            }
        });
        sender
    };
}
