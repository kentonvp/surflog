use log::{debug, error};
use std::io::{self, Write};

pub(crate) fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    if let Err(e) = io::stdout().flush() {
        error!("failed to flush stdout: {}", e);
    }
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        error!("failed to read line: {}", e);
    } else {
        input = input.trim().to_string();
    }
    debug!("User input: '{}'", input);
    input
}
