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
                if let Ok(mut file) = File::create(&path) {
                    match File::write(&mut file, "print(\"Hello, Ego!\")".as_bytes()) {
                        Ok(_) => {}
                        Err(_) => {
                            println!(" ⅹ Failed to create main.ego file");
                            std::process::exit(1);
                        }
                    }
                }

                // Create config file
                path.pop();
                path.push("ego.toml");

                match File::create(&path) {
                    Ok(mut file) => {
                        if let Some(parent) = path.parent() {
                            if let Some(dir_name) = parent.file_name() {
                                let config_data = format!(
                                    "[package]\nego_version = \"0.0.1\"\npackage_name = \"{}\"\nversion = \"1.0.0\"",
                                    dir_name.to_string_lossy()
                                );
                                if let Err(e) = file.write_all(config_data.as_bytes()) {
                                    println!(" ⅹ Failed to write to ego.toml: {}", e);
                                    std::process::exit(1);
                                }
                            } else {
                                println!(" ⅹ Failed to retrieve directory name for config file");
                                std::process::exit(1);
                            }
                        } else {
                            println!(" ⅹ Failed to retrieve parent directory for config file");
                            std::process::exit(1);
                        }
                    }
                    Err(_) => {
                        println!(" ⅹ Failed to create ego.toml");
                        std::process::exit(1);
                    }
                }

                println!(" ⚈ Succesfully initialized: {}\n", self.args[0].clone())
            }
            Err(_) => println!(" ⅹ Failed to create new ego project"),
        }
    }
}
