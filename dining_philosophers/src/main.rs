use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

struct Fork(Arc<Mutex<bool>>);

struct Philosopher {
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

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // 创建叉子
    let forks: Vec<_> = (0..5).map(|_| Fork(Arc::new(Mutex::new(false)))).collect();

    // 收集所有的 Receiver
    let mut receivers = Vec::new();

    // 创建哲学家
    let philosophers: Vec<_> = forks
        .windows(2)
        .enumerate()
        .map(|(i, pair)| {
            let (left_fork, right_fork) = (&pair[0].0, &pair[1].0);
            let (tx, rx) = std::sync::mpsc::channel();
            receivers.push(rx);
            Arc::new(Mutex::new(Philosopher::new(
                PHILOSOPHERS[i].to_string(),
                left_fork.clone(),
                right_fork.clone(),
                Mutex::new(tx.clone()),
            )))
        })
        .collect();

    // 启动哲学家线程
    let handles: Vec<_> = philosophers
        .iter()
        .map(|p| {
            let philosopher = p.clone();
            thread::spawn(move || {
                let philosopher = philosopher.lock().unwrap();
                for _ in 0..100 {
                    philosopher.think();
                    philosopher.eat();
                }
            })
        })
        .collect();

    // 等待所有哲学家线程结束
    for handle in handles {
        handle.join().unwrap();
    }
}
