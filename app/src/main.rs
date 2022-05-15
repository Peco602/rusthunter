
use std::process;

use rusthunter::options::Options;
use rusthunter::message::error;
use rusthunter::execute;

fn main() {
    let options = Options::new().unwrap_or_else(|err| {
        error(&format!("Problem parsing options: {}", err));
        process::exit(1);
    });

    if let Err(e) = execute(&options) {
        error(&format!("Application error: {}", e));
        process::exit(1);
    }
}

// rustup target add x86_64-pc-windows-gnu
// rustup toolchain install stable-x86_64-pc-windows-gnu
// cargo build --target x86_64-pc-windows-gnu

// rustup target add aarch64-unknown-linux-gnu
// cargo build --target aarch64-unknown-linux-gnu

// https://github.com/KodrAus/rust-cross-compile

