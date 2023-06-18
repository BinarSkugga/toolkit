use crate::commands::utils::{CommandResult, Executable};

pub struct Clear;

impl Executable for Clear {
    fn execute(&self, _: &[&str]) -> CommandResult {
        clearscreen::clear().unwrap();

        return CommandResult::default();
    }
}