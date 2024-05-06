mod ast;
mod commands;
mod core;
mod runtime;

use commands::Command;

fn main() {
    let command = Command::parse();
    command.exec();
}
