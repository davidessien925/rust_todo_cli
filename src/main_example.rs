use std::{
    collections::HashMap,
    io::stdin,
    // io::Read,
    // str::FromStr
};

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Saved successfully"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' does not exist", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved successfully"),
                Err(why) => println!("An error occurred; {}", why),
            },
        }
    };
}

struct Todo {
    //use Rust's built-in Hashmap to store key/value pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        //insert a new item into our map
        //we pass true as value
        self.map.insert(key, true);
    }

    //Text version
    // fn save(self) -> Result<(), std::io::Error> {
    //     let mut content = String::new();
    //     for (key, value) in self.map {
    //         let record = format!("{}\t{}\n", key, value);
    //         content.push_str(&record);
    //     }
    //     std::fs::write("db.txt", content)
    // }

    //JSON version
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    //Text version
    // fn new() -> Result<Todo, std::io::Error> {
    //     let mut f = std::fs::OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .read(true)
    //         .open("db.txt")?;

    //     let mut content = String::new();
    //     f.read_to_string(&mut content)?;
    //     let map: HashMap<String, bool> = content
    //         .lines()
    //         .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
    //         .map(|v| (v[0], v[1]))
    //         .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
    //         .collect();
    //     Ok(Todo { map })
    // }

    //JSON version
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error ocurred; {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
