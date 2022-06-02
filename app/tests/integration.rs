use std::process;

use rusthunter::options::{ Options, Mode };
use rusthunter::execute;
use rusthunter::utils::error;

#[test]
fn test_list() {
    let options = Options {
        mode: Mode::List,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        merging_directory: String::new(),
        initial_file: String::new(),
        current_file: String::new(),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}

#[test]
fn test_run() {
    let options = Options {
        mode: Mode::Run,
        verbose: false,
        config: String::from("tests/config.test"),
        binary_directory: String::new(),
        merging_directory: String::new(),
        initial_file: String::new(),
        current_file: String::new(),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}

#[test]
fn test_merge() {
    let options = Options {
        mode: Mode::Merge,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        merging_directory: String::from("tests/merging_directory"),
        initial_file: String::new(),
        current_file: String::new(),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}

#[test]
fn test_compare_full() {
    let options = Options {
        mode: Mode::Compare,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        merging_directory: String::new(),
        initial_file: String::from("tests/compare_directory/initial_snapshot.json"),
        current_file: String::from("tests/compare_directory/current_snapshot.json"),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}