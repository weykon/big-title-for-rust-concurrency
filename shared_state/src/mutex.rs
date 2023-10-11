use std::sync::Mutex;

pub fn main() {
    let v = Mutex::new(vec![10, 20, 30]);

    println!("v = {:?}", v.lock().unwrap());

    {
        let mut v = v.lock().unwrap();
        v.push(40);
    }

    println!("v = {:?}", v.lock().unwrap());
}
