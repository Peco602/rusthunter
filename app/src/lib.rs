pub mod options;
pub mod config;
pub mod plugins;
pub mod message;
pub mod validator;

use std::{
    fs::{self, File},
    io::Write,
    // io,
    // process as proc,
};
use serde_json::{Value, Map};
// use json_diff::{
//     constants::Message,
//     ds::{key_node::KeyNode, mismatch::Mismatch},
//     compare_jsons,
// };
use json_structural_diff::{colorize, JsonDiff};
use glob::glob;
use chrono::prelude::*;

use crate::options::{Options, Mode};
use crate::config::Config;
use crate::message::{info, success, warning};
use crate::plugins::{Plugin, os};
use crate::plugins::linux::{
    users::LinuxUsers,
    tcp_listen::LinuxTCPListen,
    //udp_listen::LinuxUDPListen,
    };
use crate::plugins::windows::{
    users::WindowsUsers,
    administrators::WindowsAdministrators,
    tcp_listen::WindowsTCPListen,
    udp_listen::WindowsUDPListen,
    autoruns::WindowsAutoruns,
    yara::WindowsYara,
    };

pub fn execute(options: &Options) -> Result<(), String> {
    // Plugin list
    let linux_users = LinuxUsers::new();
    let linux_tcp_listen = LinuxTCPListen::new();
    //let linux_udp_listen = LinuxUDPListen::new();
    let windows_users = WindowsUsers::new();
    let windows_administrators = WindowsAdministrators::new();
    let windows_tcp_listen = WindowsTCPListen::new();
    let windows_udp_listen = WindowsUDPListen::new();
    let windows_autoruns = WindowsAutoruns::new();
    let windows_yara = WindowsYara::new();
    let plugins: Vec<&dyn Plugin> = vec![
                                            &linux_users,
                                            &linux_tcp_listen,
                                            &windows_users,
                                            &windows_administrators,
                                            &windows_tcp_listen,
                                            &windows_udp_listen,
                                            &windows_autoruns,
                                            &windows_yara,
                                        ];

    match options.mode {
        Mode::List => list(&plugins),
        Mode::Run => run(&plugins, &options.config, &options.binary_directory, &options.verbose),
        Mode::Merge => merge(&options.merging_directory, &options.verbose),
        Mode::Compare => compare(&options.initial_file, &options.current_file),
    }
}

pub fn list(plugins: &Vec<&dyn Plugin>) -> Result<(), String> {
    println!("{: <32} {: <50} {}", "Name", "Data", "Operating System");
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
        Err(e) => return Err(format!("Problem parsing config: {}", e)),
    };

    // Plugin execution
    let mut plugins_data : Map::<String, Value> = Map::new();
    let os = os();
    for p in plugins.iter()
                    // OS-specific plugins
                    .filter(|p| p.os() == os)
                    // Enabled plugins
                    .filter(|p| config.is_plugin_enabled(p.name())) {
                        match p.run(&config, binary_directory) {
                            Ok(value) =>  {
                                plugins_data.insert(p.name().to_string(), value);
                                info(&format!("Plugin {} correctly executed", p.name()));
                            },
                            Err(e) => {
                                warning(&format!("Plugin {} failed: {}", p.name(), e));
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
    // if let Some(hostname) = hostname::get_hostname() {
    //     output_json(&host_data, format!("{}.json", &hostname))
    // } else {
    //     return Err("Cannot get hostname".to_string());
    // }
    let snapshot_filename: String = "snapshot.json".to_string();
    info(&format!("Snapshot file created: {}", snapshot_filename));
    output_json(&host_data, snapshot_filename, verbose)
}

pub fn merge(merging_directory: &String, verbose: &bool) -> Result<(), String> {
    let merging_glob = merging_directory.clone() + &"/*.json".to_string(); 
    
    let files = match glob(merging_glob.as_str()) {
        Ok(files) => files,
        Err(e) => return Err(format!("Error during file reading from merging directory: {}", e)), 
    };

    let mut merged_data : Map::<String, Value> = Map::new();
    for entry in files {
        match entry {
            Ok(path) => {
                info(&format!("Reading file: {:?}", path.display()));
                match fs::read_to_string(path) {
                    Ok(read_data) => {
                        match serde_json::from_str(&read_data) {
                           Ok(data) => {
                            match data {
                                Value::Object(m) => {
                                    for (k, v) in m  {
                                        merged_data.insert(k.clone(), v.clone());
                                    }
                                }
                                _ => {},
                            }
                           },
                           Err(e) => return Err(format!("Error during data merging: {}", e))
                        }
                    },
                    Err(e) => return Err(format!("Error during data reading: {}", e)),
                };
            },
            Err(e) => return Err(format!("Error during data reading: {}", e)),
        }
    }


    let local = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let merged_snapshots_filename: String = format!("snapshot-{}.json", local);
    //output_json(&serde_json::Map::<String, Value>(merged_data), format!("output-{}.json", local));
    info(&format!("Merged snapshots file: {}", merged_snapshots_filename));
    output_json(&merged_data, merged_snapshots_filename, verbose)

}

fn output_json(data: &Map::<String, Value>, filename: String, verbose: &bool) -> Result<(), String> {
    let output_data;
    match serde_json::to_string_pretty(&data) {
        Ok(data) => output_data = data,
        Err(e) => return Err(format!("Error during data jsonify: {}", e)),
    };

    let mut output_file;
    match File::create(filename) {
        Ok(file) => output_file = file,
        Err(e) => return Err(format!("Error during output file creation: {}", e)),
    }

    if *verbose {
        println!("{}", output_data);
    }

    match output_file.write_all(output_data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error during output file writing: {}", e)), 
    }
}


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

    // match JsonDiff::diff_string(&initial_json, &current_json, false) {
    //     Some(s) => println!("{}", s),
    //     _ => println!("No difference"),
    // };

    let json_diff = JsonDiff::diff(&initial_json, &current_json, false);
    match json_diff.diff {
        Some(result) => {
            let json_string = colorize(&result, true);
            println!("{}", json_string);
        },
        _ => success(&"No differences"),
    };

    // let mismatch = match compare_jsons(initial_data.as_str(), current_data.as_str()) {
    //     Ok(mismatch) => mismatch,
    //     Err(err) => {
    //         eprintln!("{}", err);
    //         proc::exit(1)
    //     }
    // };
    // match display_output(mismatch) {
    //     Ok(_) => (),
    //     Err(err) => eprintln!("{}", err),
    // };

    Ok(())
}

// fn display_output(result: Mismatch) -> Result<(), std::io::Error> {
//     let no_mismatch = Mismatch {
//         left_only_keys: KeyNode::Nil,
//         right_only_keys: KeyNode::Nil,
//         keys_in_both: KeyNode::Nil,
//     };
//
//     let stdout = io::stdout();
//     let mut handle = io::BufWriter::new(stdout.lock());
//     Ok(if no_mismatch == result {
//         writeln!(handle, "\n{}", Message::NoMismatch)?;
//     } else {
//         match result.keys_in_both {
//             KeyNode::Node(_) => {
//                 let mut keys = Vec::new();
//                 result.keys_in_both.absolute_keys(&mut keys, None);
//                 writeln!(handle, "\n{}:", Message::Mismatch)?;
//                 for key in keys {
//                     writeln!(handle, "{}", key)?;
//                 }
//             }
//             KeyNode::Value(_, _) => writeln!(handle, "{}", Message::RootMismatch)?,
//             KeyNode::Nil => (),
//         }
//         match result.left_only_keys {
//             KeyNode::Node(_) => {
//                 let mut keys = Vec::new();
//                 result.left_only_keys.absolute_keys(&mut keys, None);
//                 writeln!(handle, "\n{}:", Message::LeftExtra)?;
//                 for key in keys {
//                     writeln!(handle, "{}", key.red().bold())?;
//                 }
//             }
//             KeyNode::Value(_, _) => (),
//             KeyNode::Nil => (),
//         }
//         match result.right_only_keys {
//             KeyNode::Node(_) => {
//                 let mut keys = Vec::new();
//                 result.right_only_keys.absolute_keys(&mut keys, None);
//                 writeln!(handle, "\n{}:", Message::RightExtra)?;
//                 for key in keys {
//                     writeln!(handle, "{}", key.green().bold())?;
//                 }
//             }
//             KeyNode::Value(_, _) => (),
//             KeyNode::Nil => (),
//         }
//     })
// }