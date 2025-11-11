use std::fs;

use rusthunter::constants::*;
use rusthunter::execute;
use rusthunter::options::{Mode, Options};

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
        selected_plugin: String::new(),
    };

    let result = execute(&options);
    assert!(result.is_ok(), "List command should succeed");
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
        selected_plugin: String::new(),
    };

    let result = execute(&options);
    assert!(result.is_ok(), "Run command should succeed");

    // Verify snapshot file was created
    let snapshot_files: Vec<_> = fs::read_dir(".")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with(DEFAULT_SNAPSHOT_TAG) && name.ends_with(".json")
        })
        .collect();

    assert!(
        !snapshot_files.is_empty(),
        "At least one snapshot file should be created"
    );

    // Cleanup created snapshot files
    for file in snapshot_files {
        let _ = fs::remove_file(file.path());
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
        selected_plugin: String::new(),
    };

    let result = execute(&options);
    assert!(result.is_ok(), "Merge command should succeed");

    // Verify merged file was created
    let merged_files: Vec<_> = fs::read_dir(".")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("test_") && name.ends_with(".json")
        })
        .collect();

    assert!(
        !merged_files.is_empty(),
        "Merged snapshot file should be created"
    );

    // Cleanup
    for file in merged_files {
        let _ = fs::remove_file(file.path());
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
        selected_plugin: String::new(),
    };

    let result = execute(&options);
    assert!(result.is_ok(), "Compare stats command should succeed");
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
        selected_plugin: String::new(),
    };

    let result = execute(&options);
    assert!(result.is_ok(), "Compare full command should succeed");
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
        selected_plugin: String::new(),
    };

    let result = execute(&options);
    assert!(result.is_ok(), "Compare with host filter should succeed");
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
        selected_plugin: String::from("PLUGIN1"),
    };

    let result = execute(&options);
    assert!(result.is_ok(), "Compare with plugin filter should succeed");
}

#[test]
fn compare_nonexistent_file() {
    let options = Options {
        mode: Mode::Compare,
        verbose: false,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from(DEFAULT_SNAPSHOT_TAG),
        merging_directory: String::new(),
        initial_file: String::from("/nonexistent/file.json"),
        current_file: String::from("tests/compare_directory/current_snapshot.json"),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new(),
    };

    let result = execute(&options);
    assert!(result.is_err(), "Compare with nonexistent file should fail");
    assert!(result
        .unwrap_err()
        .contains("Error during initial snapshot file reading"));
}

#[test]
fn merge_nonexistent_directory() {
    let options = Options {
        mode: Mode::Merge,
        verbose: false,
        config: String::new(),
        binary_directory: String::new(),
        snapshot_tag: String::from("test"),
        merging_directory: String::from("/nonexistent/directory"),
        initial_file: String::new(),
        current_file: String::new(),
        stats: false,
        selected_host: String::new(),
        selected_plugin: String::new(),
    };

    // Should succeed but create empty merged file
    let result = execute(&options);
    assert!(
        result.is_ok(),
        "Merge with nonexistent directory should succeed with empty result"
    );

    // Cleanup any created files
    let merged_files: Vec<_> = fs::read_dir(".")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("test_") && name.ends_with(".json")
        })
        .collect();

    for file in merged_files {
        let _ = fs::remove_file(file.path());
    }
}
