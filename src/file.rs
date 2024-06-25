use savefile::{load_file, save_file};

use crate::todo::Todos;

const DATA_DIR: &str = "data/";

pub const TODO_DATA_FILE: &str = "todo";

pub fn save_data(data: &Todos, name: &str) {
    match save_file(format!("{}{}.bin", DATA_DIR, name), 0, data) {
        Ok(_) => println!("{}.bin has been saved!", name),
        Err(err) => println!("Failed to save {}.bin\nErr : {:?}", name, err),
    }
}

pub fn load_data(name: &str) -> Todos {
    match load_file(format!("{}{}.bin", DATA_DIR, name), 0) {
        Ok(data) => {
            println!("Data loaded from {}.bin", name);
            data
        }
        Err(err) => {
            println!(
                "Couldn't load {}.bin\nErr : {:?}\nMaking a new dataset",
                name, err
            );
            return Todos::new();
        }
    }
}
