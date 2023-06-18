use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};
use std::process::Output;
use std::str;

pub struct CommandResult {
    pub output: String,
    pub err: String,
    pub code: i32,
    pub success: bool
}

impl Default for CommandResult {
    fn default() -> CommandResult {
        return CommandResult{
            output: String::from(""),
            err: String::from(""),
            code: 0,
            success: true
        }
    }
}

impl CommandResult {
    pub fn error(message: &str, code: i32) -> CommandResult {
        return CommandResult{
            output: String::from(""),
            err: String::from(message),
            code, success: false
        }
    }

    pub fn success(message: &str) -> CommandResult {
        return CommandResult{
            output: String::from(message),
            err: String::from(""),
            code: 0, success: true
        }
    }
}

impl Display for CommandResult {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut to_print = if self.success {self.output.to_string()} else {self.err.to_string()};

        if !to_print.ends_with("\n") && to_print.len() > 0 {
            to_print.push_str("\n");
        }

        if !self.success {
            to_print = red(to_print.as_str());
        }

        write!(f, "{}", to_print)
    }
}

pub trait ToCommandResult {
    fn to_result(&self) -> CommandResult;
}

impl ToCommandResult for Output {
    fn to_result(&self) -> CommandResult {
        let output = str::from_utf8(&self.stdout).expect("").to_string();
        let err = str::from_utf8(&self.stderr).expect("").to_string();
        let code = self.status.code().expect("");

        return CommandResult{output, err, code, success: code == 0};
    }
}

pub trait Executable {
    fn execute(&self, args: &[&str]) -> CommandResult;
    fn empty_execute(&self) -> CommandResult {
        return self.execute(&[]);
    }
}

pub fn take_input(prompt: &str) -> String {
    print!("{prompt}");

    let mut str_inputted = String::new();
    let _ = stdout().flush();

    stdin().read_line(&mut str_inputted).expect("Did not enter a correct string");
    match str_inputted.chars().next_back() {
        Some('\n') => str_inputted.pop(),
        Some('\r') => str_inputted.pop(),
        _ => Option::None
    };

    return str_inputted;
}

pub fn green(text: &str) -> String {
    return format!("\u{1b}[92m{}\u{1b}[0m", text);
}

pub fn red(text: &str) -> String {
    return format!("\u{1b}[91m{}\u{1b}[0m", text);
}
