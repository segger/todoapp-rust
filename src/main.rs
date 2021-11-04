use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Vad?");
    let item = std::env::args().nth(2).expect("Med vad?");

//    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Couldn't init db.txt");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(e) => println!("an error occured: {}", e),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(e) => println!("an error occured: {}", e),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}


