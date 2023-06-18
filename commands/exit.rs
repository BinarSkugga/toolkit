use std::process::exit;
use crate::commands::utils::{CommandResult, Executable};

pub struct Exit;

impl Executable for Exit {
    fn execute(&self, _: &[&str]) -> CommandResult {
        exit(0);
    }
}