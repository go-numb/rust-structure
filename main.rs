use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher{
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating apple.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("------------> {} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    // 並列処理
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}