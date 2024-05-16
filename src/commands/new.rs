use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub struct New {
    args: Vec<String>,
}

impl New {
    pub fn new(args: Vec<String>) -> New {
        New { args }
    }
    pub fn exec(&self) {
        println!("\n ◔ Creating new ego project");

        let mut path = PathBuf::from(self.args[0].clone());
        match fs::create_dir_all(&path) {
            Ok(_) => {
                // Create main file
                path.push("main.ego");
                if let Ok(mut file) = File::create(path) {
                    match File::write(&mut file, "print(\"Hello, Ego!\")".as_bytes()) {
                        Ok(_) => println!(" ⚈ Succesfully initialized: {}\n", self.args[0].clone()),
                        Err(_) => println!(" Failed to create new ego project"),
                    }
                }
            }
            Err(_) => println!(" Failed to create new ego project"),
        }
    }
}
