use serde_json::{Value, Map};
use chrono::prelude::Local;

use crate::constants::{SNAPSHOT_EXTENSION};
use crate::config::Config;
use crate::utils::{print_info, print_success, print_warning, output_json};
use crate::plugins::{Plugin, os};

pub fn list(plugins: &Vec<&dyn Plugin>) -> Result<(), String> {
    println!("{: <32} {: <50} {}", "Plugin", "Description", "Operating System");
    println!("{:=<100}","=");
    for p in plugins {
        p.show();
    }
    Ok(())
}

pub fn run(plugins: &Vec<&dyn Plugin>, config_file: &str, binary_directory: &str, snapshot_tag: &str, verbose: &bool) -> Result<(), String> {
    // Config loading
    let config = match Config::new(config_file) {
        Ok(config) => config,
        Err(e) => return Err(format!("Error parsing config: {}", e)),
    };

    // Plugin execution
    let mut plugins_data : Map::<String, Value> = Map::new();
    let os = os();
    for p in plugins.iter()
                    // OS-specific plugins
                    .filter(|p| p.os() == os)
                    // Enabled plugins
                    .filter(|p| config.is_plugin_enabled(p.name())) {
                        print_info(&format!("Executing {} plugin", p.name()));
                        match p.run(&config, binary_directory) {
                            Ok(value) =>  {
                                plugins_data.insert(p.name().to_string(), value);
                                print_success("Done");
                            },
                            Err(e) => {
                                print_warning(&format!("Failed: {}", e));
                            },
                        };
                    }

    // Data wrapping & output
    let mut host_data : Map::<String, Value> = Map::new();
    if let Some(hostname) = hostname::get_hostname() {
        host_data.insert(hostname.clone(), serde_json::Value::Object(plugins_data));

        let local_time = Local::now().format("%Y%m%d-%H%M%S").to_string();
        let snapshot_filename: String = format!("{}_{}_{}.{}", snapshot_tag, hostname, local_time, SNAPSHOT_EXTENSION);
        print_success(&format!("Snapshot file created: {}", snapshot_filename));
        output_json(&host_data, snapshot_filename, verbose)
    } else {
        return Err("Cannot get hostname".to_string());
    }
}