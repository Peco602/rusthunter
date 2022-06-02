use crate::constants::*;
use crate::config::Config;
use crate::utils::{info, success, warning, output_json};
use crate::plugins::{Plugin, os};
use serde_json::{Value, Map};

pub fn list(plugins: &Vec<&dyn Plugin>) -> Result<(), String> {
    println!("{: <32} {: <50} {}", "Plugin", "Description", "Operating System");
    println!("{:=<100}","=");
    for p in plugins {
        p.show();
    }
    Ok(())
}

pub fn run(plugins: &Vec<&dyn Plugin>, config_file: &str, binary_directory: &str, verbose: &bool) -> Result<(), String> {
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
                        info(&format!("Executing {} plugin", p.name()));
                        match p.run(&config, binary_directory) {
                            Ok(value) =>  {
                                plugins_data.insert(p.name().to_string(), value);
                                success("Done");
                            },
                            Err(e) => {
                                warning(&format!("Failed: {}", e));
                            },
                        };
                    }

    // Data wrapping
    let mut host_data : Map::<String, Value> = Map::new();
    if let Some(hostname) = hostname::get_hostname() {
        host_data.insert(hostname, serde_json::Value::Object(plugins_data));
    } else {
        return Err("Cannot get hostname".to_string());
    }

    // // Data output
    let snapshot_filename: String = format!("{}.{}", SNAPSHOT_FILENAME, SNAPSHOT_EXTENSION);
    success(&format!("Snapshot file created: {}", snapshot_filename));
    output_json(&host_data, snapshot_filename, verbose)
}