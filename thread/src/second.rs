use std::{thread, time::Duration};

pub fn second() {
    println!("[Second]: Running ");
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    println!("Not waiting the other thread done work... And leave 4 jobs did not finish");
    println!("[Second]: DONE !!!");
}
