pub static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

pub struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<bool>>,
    right_fork: Arc<Mutex<bool>>,
    thoughts: Mutex<Sender<String>>,
}

impl Philosopher {
    fn new(
        name: String,
        left_fork: Arc<Mutex<bool>>,
        right_fork: Arc<Mutex<bool>>,
        thoughts: Mutex<Sender<String>>,
    ) -> Philosopher {
        Philosopher {
            name,
            left_fork,
            right_fork,
            thoughts,
        }
    }

    fn think(&self) {
        println!("{} is thinking", &self.name);
        thread::sleep(Duration::from_millis(10));
        self.thoughts
            .lock()
            .unwrap()
            .send(format!("{} finished thinking", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        let left_fork = self.left_fork.lock().unwrap();
        let right_fork = self.right_fork.lock().unwrap();

        println!("{} is eating", &self.name);
        thread::sleep(Duration::from_millis(10));
        self.thoughts
            .lock()
            .unwrap()
            .send(format!("{} finished eating", &self.name))
            .unwrap();
    }
}
