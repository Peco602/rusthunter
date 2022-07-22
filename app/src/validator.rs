use regex::Regex;

fn check_regex(regex: &str, text: &str) -> bool {
    Regex::new(regex).unwrap().is_match(text)
}

pub fn validate_windows_path(text: &str) -> bool {
    check_regex(r"^([a-z]:)?((?:[\\]?(?:[\w !#()-]+|[.]{1,2})+)*[\\])?$", text)
}

pub fn validate_windows_sam_account_name(text: &str) -> bool {
    // https://devblogs.microsoft.com/scripting/powertip-how-to-detect-a-valid-active-directory-user-name-using-regular-expressions/
    check_regex(r"^[^/\\\[\]:;|=,+?<>]+$", text)
}

pub fn validate_linux_path(text: &str) -> bool {
    check_regex(r"^/(?:[^/]+/)*$", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_path() {
        assert_eq!(validate_windows_path(r"c:\"), true);
        assert_eq!(validate_windows_path(r"c:\users\"), true);
        assert_eq!(validate_windows_path(r"c:\users\application data\"), true);
        assert_eq!(validate_windows_path(r"c:\users\test"), false);
        assert_eq!(validate_windows_path(r"c:\users\test.exe"), false);
        assert_eq!(validate_windows_path(r"c:\users\test && whoami"), false);
    }

    #[test]
    fn windows_sam_account_name() {
        assert_eq!(validate_windows_sam_account_name(r"domain admins"), true);
        assert_eq!(validate_windows_sam_account_name(r"enterprise admins"), true);
        assert_eq!(validate_windows_sam_account_name(r"management-users"), true);
        assert_eq!(validate_windows_sam_account_name(r"[group]"), false);
        assert_eq!(validate_windows_sam_account_name(r"users+groups"), false);
        assert_eq!(validate_windows_sam_account_name(r"group:"), false);
    }

    #[test]
    fn linux_path() {
        assert_eq!(validate_linux_path(r"/"), true);
        assert_eq!(validate_linux_path(r"/root/"), true);
        assert_eq!(validate_linux_path(r"/home/Desktop/"), true);
        assert_eq!(validate_linux_path(r"/root"), false);
        assert_eq!(validate_linux_path(r"/root/ && whoami"), false);
        assert_eq!(validate_linux_path(r"/root/; whoami"), false);
    }
}