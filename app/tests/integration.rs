use std::process;

use rusthunter::options::{ Options, Mode };
use rusthunter::execute;
use rusthunter::message::error;

#[test]
fn run() {
    let options = Options {
        mode: Mode::Run,
        verbose: true,
        config: String::from("tests/config.test"),
        binary_directory: String::new(),
        merging_directory: String::new(),
        initial_file: String::new(),
        current_file: String::new()
    };

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}


#[test]
fn list() {
    let options = Options {
        mode: Mode::List,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        merging_directory: String::new(),
        initial_file: String::new(),
        current_file: String::new()
    };

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}