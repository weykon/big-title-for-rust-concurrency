// 像花饼一样，左右脚

use std::sync::{Arc, Mutex};

struct Person { 
    left : Arc<Mutex<bool>>,
    right : Arc<Mutex<bool>>,
}

fn main() {
    println!("Hello, world!");
}
