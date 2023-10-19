mod printer;
use std::{
    sync::{mpsc, Arc},
    thread,
};

use printer::Printer;
fn main() {
    let printer = Printer::new();
    printer.print();

    let (tx, rx) = mpsc::channel();

 
    fn work_thread() {}

    for _ in 0..5 {
        let sender = tx.clone();
        let thread_count = Arc::clone(&printer.thread_count);
        thread::spawn(move || {
            sender.send(1).unwrap();
            {
                let mut thread_count = thread_count.lock().unwrap();
                *thread_count += 1;
            }
            work_thread();
            {
                let mut thread_count = thread_count.lock().unwrap();
                *thread_count -= 1;
            }
        });
    }
}
