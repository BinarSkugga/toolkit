use crate::commands::utils::{CommandResult, Executable};

pub struct Hello;

impl Executable for Hello {
    fn execute(&self, _: &[&str]) -> CommandResult {
        return CommandResult::success("Hi :)")
    }
}