use std::collections::HashMap;

mod commands;
use commands::install::{Install};
use crate::commands::clear::{Clear};
use crate::commands::exit::{Exit};
use crate::commands::hello::{Hello};
use crate::commands::utils::{CommandResult, Executable, green, take_input};


fn main() {
    let version = "0.1.0";
    let mut command_db: HashMap<&str, Box<dyn Executable>> = HashMap::new();
    command_db.insert("clear", Box::new(Clear{}));
    command_db.insert("exit", Box::new(Exit{}));
    command_db.insert("hello", Box::new(Hello{}));
    command_db.insert("install", Box::new(Install::new("dnf")));

    let alias_db = HashMap::from([
        ("cls", "clear"),
        ("bye", "exit")
    ]);

    command_db.get("clear").unwrap().empty_execute();
    println!("{}", green(format!("== Toolkit v{} ==", version).as_str()));
    loop {
        let input = take_input(&green("TK> "));
        let args: Vec<&str> = input.trim().split(" ").collect();
        let command = args.get(0).unwrap();

        let result = match command_db.get(command) {
            // Command was found
            Some(command) => command.execute(args.as_slice()),
            None => {
                if input.trim() == "" {
                    continue
                }

                let result;
                if alias_db.contains_key(command) {
                    let aliased_command = alias_db.get(command).unwrap();
                    result = command_db.get(aliased_command).unwrap().execute(args.as_slice());
                } else {
                    result = CommandResult::error(format!("Command '{input}' not found.").as_str(), 1);
                }

                result
            }
        };

        print!("{result}");
    }
}