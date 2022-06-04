use serde_json::Value;
use assert_json_diff;
use diffy::{create_patch, PatchFormatter};
use std::fs;

use crate::utils::{
    success,
    warning,
    error
};

pub fn compare(initial_file: &String, current_file: &String, stats: &bool, selected_host: &String, selected_plugin: &String) -> Result<(), String> {
    // Snapshot file reading
    let initial_data;
    match fs::read_to_string(initial_file) {
        Ok(data) => initial_data = data,
        Err(e) => return Err(format!("Error during initial snapshot file reading: {}", e)),
    };

    let current_data;
    match fs::read_to_string(current_file) {
        Ok(data) => current_data = data,
        Err(e) => return Err(format!("Error during current snapshot file reading: {}", e)),
    };
   
    // Snapshot data parsing
    let initial_json: Value;
    match serde_json::from_str(&initial_data) {
        Ok(json) => initial_json = json,
        Err(e) => return Err(format!("Error during initial snapshot data parsing: {}", e)),
    }

    let current_json: Value;
    match serde_json::from_str(&current_data) {
        Ok(json) => current_json = json,
        Err(e) => return Err(format!("Error during current snapshot data parsing: {}", e)),
    }

    // Show comparison statistics
    if *stats {
        show_statistics(&initial_json, &current_json);
    } else {
        // Show differences
        if let Err(e) = show_differences(&initial_json, &current_json, selected_host, selected_plugin) {
            return Err(format!("Error during snapshot comparison: {}", e))
        }
    }

    Ok(())
}

fn show_statistics(initial_json: &Value, current_json: &Value) {
    let mut initial_value;
    let mut current_value;
    let mut message: colored::ColoredString;

    println!("{: <32} {: <32} {: <10} {: <10}", "Host", "Plugin", "Initial", "Current");
    println!("{:=<100}","=");

    // Outer loop over HOSTS
    for (host_name, initial_host_data) in initial_json.as_object().unwrap() {
        // Inner loop over PLUGINS
        for (plugin_name, initial_plugin_data) in initial_host_data.as_object().unwrap() {
            initial_value = initial_plugin_data.as_array().unwrap().len();
            match ((current_json[host_name])[plugin_name]).as_array() {
                    Some(v) => {
                        current_value = v.len();
                        match assert_json_diff::assert_json_matches_no_panic(initial_plugin_data, v, assert_json_diff::Config::new(assert_json_diff::CompareMode::Strict)) {
                            Ok(_) => message = success("Ok"),
                            Err(_) => message = warning("Mismatch"),
                        }
                    },
                    None => {
                        current_value = 0;
                        message = error("Not found")
                    },
                };
            println!("{: <32} {: <32} {: <10} {: <10} {: <20}", host_name, plugin_name, initial_value, current_value, message);
        }
        println!("{:-<100}","-");
    }
}

fn show_differences(initial_json: &Value, current_json: &Value, selected_host: &str, selected_plugin: &str) -> Result<(), String> {
    let mut initial_json_x: &Value = initial_json;
    let mut current_json_x: &Value = current_json;
    
    if selected_host != "" {
        if (initial_json_x[selected_host]).is_null() {
            return Err(format!("Host {} not found in initial snaphost", selected_host));
        } else {
            initial_json_x = &initial_json_x[selected_host];
        }
        if (current_json_x[selected_host]).is_null() {
            return Err(format!("Host {} not found in current snaphost", selected_host));
        } else {
            current_json_x = &current_json_x[selected_host];
        }
    }

    if selected_plugin != "" {
        if (initial_json_x[selected_plugin]).is_null() {
            return Err(format!("Plugin {} not found for host {} in initial snaphost", selected_plugin, selected_host));
        } else {
            initial_json_x = &initial_json_x[selected_plugin];
        }
        if (current_json_x[selected_plugin]).is_null() {
            return Err(format!("Plugin {} not found for host {} in current snaphost", selected_plugin, selected_host));
        } else {
            current_json_x = &current_json_x[selected_plugin];
        }
    }

    let initial_data_x;
    match serde_json::to_string_pretty(&initial_json_x) {
        Ok(data) => initial_data_x = data,
        Err(e) => return Err(format!("Error during initial data jsonify: {}", e)),
    };

    let current_data_x;
    match serde_json::to_string_pretty(&current_json_x) {
        Ok(data) => current_data_x = data,
        Err(e) => return Err(format!("Error during initial data jsonify: {}", e)),
    };

    let patch = create_patch(&initial_data_x, &current_data_x);
    let f = PatchFormatter::new().with_color();
    print!("{}", f.fmt_patch(&patch));

    Ok(())
}