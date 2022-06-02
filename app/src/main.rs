
use std::process;

use rusthunter::options::Options;
use rusthunter::utils::print_error;
use rusthunter::execute;

fn main() {
    let options = Options::new().unwrap_or_else(|err| {
        print_error(&format!("Error during options parsing: {}", err));
        process::exit(1);
    });

    if let Err(e) = execute(&options) {
        print_error(&format!("Application print_error: {}", e));
        process::exit(1);
    }
}