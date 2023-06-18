use std::process::{Command};
use std::str;

use crate::commands::utils::{CommandResult, Executable, ToCommandResult};

pub struct Install {
    package_manager: String
}

impl Install {
    pub fn new(package_manager: &str) -> Install {
        return Install {
            package_manager: String::from(package_manager)
        }
    }
}

impl Executable for Install {
    fn execute(&self, args: &[&str]) -> CommandResult {
        if args.len() == 1 {
            return CommandResult::error("No package name provided.", 1);
        }

        let output = Command::new(self.package_manager.as_str())
            .arg("install")
            .arg(&args[1])
            .output().expect("Failed to install");

        return output.to_result();
    }
}
