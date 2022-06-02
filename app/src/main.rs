
use std::process;

use rusthunter::options::Options;
use rusthunter::utils::error;
use rusthunter::execute;

fn main() {
    let options = Options::new().unwrap_or_else(|err| {
        error(&format!("Error during options parsing: {}", err));
        process::exit(1);
    });

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}