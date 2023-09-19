use std::thread;
pub fn i_like_to_move () { 
    println!("\nI like to move it move it!");
    let a_num_at_main = 32;

    let handle = thread::spawn(move || {
        println!("here may have something {}",a_num_at_main);
    });

    handle.join().unwrap();
}