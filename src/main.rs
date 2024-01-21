use std::{collections::HashMap, fs::File, io::stdin, io::Read};

use serde_json::Value;

fn main() {
    let mut todo = Todo::new().expect("Initialisation of db failed");

    loop {
        println!("Please input your action.");
        let mut action = String::new();
        stdin().read_line(&mut action).expect("Failed to read line");
        let action = String::from(action.trim());

        if action == "showall" {
            match todo.show_all() {
                Ok(json_value) => {
                    // Convert the JSON Value to a pretty-printed string
                    let pretty_json_string = serde_json::to_string_pretty(&json_value)
                        .expect("Failed to convert to JSON string");
                    println!("Pretty JSON:\n{}", pretty_json_string);

                    // Convert the JSON Value to a compact string
                    let compact_json_string = serde_json::to_string(&json_value)
                        .expect("Failed to convert to JSON string");
                    println!("Compact JSON: {}", compact_json_string);
                }
                Err(e) => println!("An error occurred: {}", e),
            }
        } else {
            println!("Please input your item.");
            let mut item = String::new();
            stdin().read_line(&mut item).expect("Failed to read line");
            let item = String::from(item.trim());

            println!("{:?}, {:?}", action, item);

            match action.as_str() {
                "add" => {
                    todo.insert(item);
                    match todo.save() {
                        Ok(_) => println!("Saved successfully"),
                        Err(why) => println!("An error occurred: {}", why),
                    }
                }
                "complete" => match todo.complete(&item) {
                    None => println!("'{}' does not exist", item),
                    Some(_) => match todo.save() {
                        Ok(_) => println!("Todo saved successfully"),
                        Err(why) => println!("An error occurred; {}", why),
                    },
                },
                _ => println!("action does not exist"),
            };
        }

        println!("Do you want to perform another action? (yes/no)");
        let mut continue_input = String::new();
        stdin()
            .read_line(&mut continue_input)
            .expect("Failed to read line");
        let continue_input = continue_input.trim().to_lowercase();

        if continue_input != "yes" {
            break;
        }
    }
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

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

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

    fn show_all(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut file: File = File::open("db.json")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let json_value = serde_json::from_str(&content)?;

        Ok(json_value)
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
