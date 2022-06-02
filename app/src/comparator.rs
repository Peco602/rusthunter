use serde_json::Value;
use diffy::{create_patch, PatchFormatter};
use std::fs;


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

    println!("{: <32} {: <32} {: <10} {: <10}", "Host", "Plugin", "Initial", "Current");
    println!("{:=<100}","=");

    // Outer loop over HOSTS
    for (host_name, initial_host_data) in initial_json.as_object().unwrap() {
        // Inner loop over PLUGINS
        for (plugin_name, initial_plugin_data) in initial_host_data.as_object().unwrap() {
            initial_value = initial_plugin_data.as_array().unwrap().len();
            current_value = match ((current_json[host_name])[plugin_name]).as_array() {
                    Some(v) => v.len(),
                    None => 0,
                };
            println!("{: <32} {: <32} {: <10} {: <10}", host_name, plugin_name, initial_value, current_value);
        }
    }
}

fn show_differences(initial_json: &Value, current_json: &Value, selected_host: &str, selected_plugin: &str) -> Result<(), String> {
    let mut initial_json_x: Value = initial_json.clone();
    let mut current_json_x: Value = current_json.clone();

    if selected_host != "" {
        match (initial_json_x[selected_host]).as_object() {
            Some(v) => initial_json_x = serde_json::Value::Object(v.clone()),
            None => return Err(format!("Host {} not found in initial snaphost", selected_host)),
        };
        match (current_json_x[selected_host]).as_object() {
            Some(v) => current_json_x = serde_json::Value::Object(v.clone()),
            None => return Err(format!("Host {} not found in current snaphost", selected_host)),
        };
    }
    
    if selected_plugin != "" {
        match (initial_json_x[selected_plugin]).as_array() {
            Some(v) => initial_json_x = serde_json::Value::Array(v.clone()),
            None => return Err(format!("Plugin {} not found for host {} in initial snaphost", selected_plugin, selected_host)),
        };
        match (current_json_x[selected_plugin]).as_array() {
            Some(v) => current_json_x = serde_json::Value::Array(v.clone()),
            None => return Err(format!("Plugin {} not found for host {} in current snaphost", selected_plugin, selected_host)),
        };
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