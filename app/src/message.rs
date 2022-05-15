use colored::Colorize;

pub fn info(message: &str) {
    eprintln!("{}", message.bold())
}

pub fn success(message: &str) {
    eprintln!("{}", message.green().bold())
}

pub fn warning(message: &str) {
    eprintln!("{}", message.yellow().bold())
}

pub fn error(message: &str) {
    eprintln!("{}", message.red().bold())
}
