use colored::{Colorize, ColoredString};

//
pub fn info(message: &str) -> ColoredString {
    format!("[*] {}", message).white()
}

pub fn print_info(message: &str) {
    eprintln!("{}", info(message))
}

pub fn success(message: &str) -> ColoredString {
    format!("[+] {}", message).green().bold()
}

pub fn print_success(message: &str) {
    eprintln!("{}", success(message))
}

pub fn warning(message: &str) -> ColoredString {
    format!("[!] {}", message).yellow().bold()
}

pub fn print_warning(message: &str) {
    eprintln!("{}", warning(message))
}

pub fn error(message: &str) -> ColoredString {
    format!("[-] {}", message).red().bold()
}

pub fn print_error(message: &str) {
    eprintln!("{}", error(message))
}

use std::{
    fs::File,
    io::Write,
};
use serde_json::{Value, Map};
pub fn output_json(data: &Map::<String, Value>, filename: String, verbose: &bool) -> Result<(), String> {
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