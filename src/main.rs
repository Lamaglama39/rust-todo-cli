use std::collections::HashMap;
use std::io::Read;

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
            // open the db file
                let mut f = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .read(true)
                    .open("db.txt")?;
                // read its content into a new string
                let mut content = String::new();
                    f.read_to_string(&mut content)?;

        // allocate an empty HashMap
        let mut map = HashMap::new();

        // loop over each lines of the file
        for entries in content.lines() {
            // split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            // insert them into HashMap
            map.insert(String::from(key), val.parse().unwrap_or(false));
        }
        // Return Ok
        Ok(Todo { map })
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    fn list(&self) {
        for (task, status) in &self.map {
            let status_text = if *status { "[ ]" } else { "[x]" };
            println!("{} {}", status_text, task);
        }
    }

    fn delete(&mut self, key: &String) -> Option<()> {
        match self.map.remove(key) {
            Some(_) => Some(()),
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

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        println!("{:?}, {:?}", action, item);
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        println!("{:?}, {:?}", action, item);
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "list" {
        println!("Todo List:");
        todo.list();
    } else if action == "delete" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        println!("{:?}, {:?}", action, item);
        match todo.delete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo deleted"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}


