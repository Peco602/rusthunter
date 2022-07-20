use std::process;

use rusthunter::constants::*;
use rusthunter::options::{ Options, Mode };
use rusthunter::execute;
use rusthunter::utils::print_error;

#[test]
fn list() {
    let options = Options {
        mode: Mode::List,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from(DEFAULT_SNAPSHOT_TAG),
        merging_directory: String::new(),
        initial_file: String::new(),
        current_file: String::new(),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}

#[test]
fn run() {
    let options = Options {
        mode: Mode::Run,
        verbose: false,
        config: String::from("tests/config.test"),
        binary_directory: String::new(),
        snapshot_tag: String::from(DEFAULT_SNAPSHOT_TAG),
        merging_directory: String::new(),
        initial_file: String::new(),
        current_file: String::new(),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}

#[test]
fn merge() {
    let options = Options {
        mode: Mode::Merge,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from("test"),
        merging_directory: String::from("tests/merging_directory"),
        initial_file: String::new(),
        current_file: String::new(),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}

#[test]
fn compare_stats() {
    let options = Options {
        mode: Mode::Compare,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from(DEFAULT_SNAPSHOT_TAG),
        merging_directory: String::new(),
        initial_file: String::from("tests/compare_directory/initial_snapshot.json"),
        current_file: String::from("tests/compare_directory/current_snapshot.json"),
        stats: true,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}

#[test]
fn compare_full() {
    let options = Options {
        mode: Mode::Compare,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from(DEFAULT_SNAPSHOT_TAG),
        merging_directory: String::new(),
        initial_file: String::from("tests/compare_directory/initial_snapshot.json"),
        current_file: String::from("tests/compare_directory/current_snapshot.json"),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}

#[test]
fn compare_host() {
    let options = Options {
        mode: Mode::Compare,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from(DEFAULT_SNAPSHOT_TAG),
        merging_directory: String::new(),
        initial_file: String::from("tests/compare_directory/initial_snapshot.json"),
        current_file: String::from("tests/compare_directory/current_snapshot.json"),
        stats: false,
        selected_host: String::from("HOST1"),
        selected_plugin: String::new()
    };

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}

#[test]
fn compare_plugin() {
    let options = Options {
        mode: Mode::Compare,
        verbose: true,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from(DEFAULT_SNAPSHOT_TAG),
        merging_directory: String::new(),
        initial_file: String::from("tests/compare_directory/initial_snapshot.json"),
        current_file: String::from("tests/compare_directory/current_snapshot.json"),
        stats: false,
        selected_host: String::from("HOST1"),
        selected_plugin: String::from("PLUGIN1")
    };

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}