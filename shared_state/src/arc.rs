use std::sync::Arc;
use std::thread;

pub fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = Arc::clone(&v);
        let handle = thread::spawn(move || {
            let thread_id = thread::current().id();
            println!(" {thread_id:?} : {v:?}");
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(
        |handle| handle.join().unwrap()
    );

    println!("v = {:?}", v);
}
