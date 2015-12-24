struct Philosopher {
    name: String,
    left: usize,
    right: usize
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        std::thread::sleep_ms(150);
        let _right = table.forks[self.right].lock().unwrap();
        
        println!("{} is eating.", self.name);

        std::thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<std::sync::Mutex<()>>,
}

fn main() {
    let table = std::sync::Arc::new(Table { forks: vec![
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(())
    ]});

    let philosophers = vec![
        Philosopher::new("Sean Connery", 0, 1),
        Philosopher::new("David Niven", 1, 2),
        Philosopher::new("George Lazenby", 2, 3),
        Philosopher::new("Roger Moore", 3, 4),
        Philosopher::new("Timothy Dalton", 4, 5),
        Philosopher::new("Pierce Brosnan", 5, 6),
        Philosopher::new("Daniel Craig", 0, 6)
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        std::thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn it_works() {
}
