use regex::Regex;

fn check_regex(regex: &str, text: &str) -> bool {
    Regex::new(regex).unwrap().is_match(text)
}

pub fn validate_windows_path(text: &str) -> bool {
    check_regex(r"^[a-zA-Z]:\\(?:\w+\\)*\w+\.\w+$", text)
}
