use regex::Regex;

fn check_regex(regex: &str, text: &str) -> bool {
    Regex::new(regex).unwrap().is_match(text)
}

pub fn validate_windows_path(text: &str) -> bool {
    check_regex(r"^([a-z]:)?((?:[\\]?(?:[\w !#()-]+|[.]{1,2})+)*[\\])?$", text)
}

pub fn validate_linux_path(text: &str) -> bool {
    check_regex(r"^/(?:[^/]+/)*$", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_windows_path() {
        assert_eq!(validate_windows_path(r"c:\"), true);
        assert_eq!(validate_windows_path(r"c:\users\"), true);
        assert_eq!(validate_windows_path(r"c:\users\application data\"), true);
        assert_eq!(validate_windows_path(r"c:\users\test"), false);
        assert_eq!(validate_windows_path(r"c:\users\test.exe"), false);
        assert_eq!(validate_windows_path(r"c:\users\test && whoami"), false);
    }

    #[test]
    fn test_validate_linux_path() {
        assert_eq!(validate_linux_path(r"/"), true);
        assert_eq!(validate_linux_path(r"/root/"), true);
        assert_eq!(validate_linux_path(r"/home/Desktop/"), true);
        assert_eq!(validate_linux_path(r"/root"), false);
        assert_eq!(validate_linux_path(r"/root/ && whoami"), false);
        assert_eq!(validate_linux_path(r"/root/; whoami"), false);
    }
}