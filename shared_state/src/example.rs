use std::{sync::Arc, sync::Mutex, thread};

pub fn main() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));

    let v2 = Arc::clone(&v);

    let handle = thread::spawn(move || {
        let mut v = v2.lock().unwrap();
        v.push(40);
    });

    {
        let mut v = v.lock().unwrap();
        v.push(50);
    }

    handle.join().unwrap();

    println!("v = {:?}", v);
}
