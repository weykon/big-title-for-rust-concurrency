use std::thread;
pub fn base() {
    println!("[Base]: Running ");
    let handle = thread::spawn(|| println!("Hello from a thread!"));

    println!("here is main thread!");
    println!("but it is waiting the other thread done work...");
    handle.join().unwrap();
    println!("main thread done work!");
    println!("[Base]: DONE!!! ");
}
