pub mod logo;
pub mod new;
pub mod run;

use self::logo::Logo;
use self::new::New;
use self::run::Run;

use crate::core::error;
use crate::core::error::ErrorType;
use std::env;

pub enum Command {
    Run(Run),
    Logo(Logo),
    New(New),
}

impl Command {
    pub fn parse() -> Command {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 2 {
            let command = args[1].clone();
            let remaining_args = &args[2..];
            return Command::cmd_from_str(command.as_str(), remaining_args.to_vec());
        } else {
            // print help message instead of error
            error::throw(
                ErrorType::EgoUsageError,
                "a command is required to execute ego",
                None,
            );
            std::process::exit(1); // to avoid types error
        };
    }
    fn cmd_from_str(command: &str, args: Vec<String>) -> Command {
        match command {
            "run" => Command::Run(Run::new(args)),
            "logo" => Command::Logo(Logo::new(args)),
            "new" => Command::New(New::new(args)),
            _ => {
                error::throw(
                    ErrorType::EgoUsageError,
                    format!("Unknown command {}", command).as_str(),
                    None,
                );
                std::process::exit(1);
            }
        }
    }
    pub fn exec(&self) {
        match self {
            Command::Run(v) => v.exec(),
            Command::Logo(v) => v.exec(),
            Command::New(v) => v.exec(),
        }
    }
}
