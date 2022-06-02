use serde_json::Value;
use diffy::{create_patch, PatchFormatter};
use std::fs;


pub fn compare(initial_file: &String, current_file: &String) -> Result<(), String> {
    let initial_data;
    match fs::read_to_string(initial_file) {
        Ok(data) => initial_data = data,
        Err(e) => return Err(format!("Error during initial snapshot reading: {}", e)),
    };

    let current_data;
    match fs::read_to_string(current_file) {
        Ok(data) => current_data = data,
        Err(e) => return Err(format!("Error during current snapshot reading: {}", e)),
    };
   
    let initial_json: Value;
    match serde_json::from_str(&initial_data) {
        Ok(json) => initial_json = json,
        Err(e) => return Err(format!("Error during initial snapshot parsing: {}", e)),
    }

    let current_json: Value;
    match serde_json::from_str(&current_data) {
        Ok(json) => current_json = json,
        Err(e) => return Err(format!("Error during current snapshot parsing: {}", e)),
    }

    show_summary(&initial_json, &current_json);

    show_differences(&initial_data, &current_data);

    Ok(())
}


fn show_differences(initial_data: &str, current_data: &str) {
    let patch = create_patch(&initial_data, &current_data);
    let f = PatchFormatter::new().with_color();
    print!("{}", f.fmt_patch(&patch));
}

fn show_summary(initial_json: &Value, current_json: &Value) {
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