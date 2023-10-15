use std::sync::{
    mpsc::{Receiver, RecvError, Sender, TryRecvError},
    Arc, Mutex,
};
use std::thread;

pub struct Printer {
    pub terminal: Receiver<String>,
    pub thread_count: Arc<Mutex<i32>>,
}

impl Printer {
    pub fn new() -> Self {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel();
        Printer {
            terminal: rx,
            thread_count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn print(&self) {
        loop {
            
            match self.terminal.try_recv() {
                Ok(output) => self.format(output),
                Err(err) => {
                    if err == TryRecvError::Disconnected {
                        break;
                    }
                }
            }
            // let count = self.thread_count.lock().unwrap();
            // if *count == -1 {
            //     break;
            // }
            use std::time::Duration;
            thread::sleep(Duration::from_millis(41));
        }
    }

    fn format(&self, output: String) {}
}
