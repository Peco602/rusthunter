use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxUsers {}

impl Plugin for LinuxUsers {
    fn name(&self) -> &str {
        &"linux_users"
    }

    fn description(&self) -> &str {
        &"Local users"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "cat /etc/passwd | cut -d: -f1 | sort";
        match self.linux_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxUsers {
    pub fn new() -> Self {
        LinuxUsers {}
    }
}

#[cfg(test)]
mod tests {
    use std::include_str;
    use serde_json::json;
    use super::*;

    #[test]
    fn test_linux_users() {
        let output = include_str!("output.txt");
        let linux_users = LinuxUsers::new();
        assert_eq!(json!(["_apt", "_rpc", "backup", "bin", "daemon", "games", "gnats", "irc", "landscape", "list", "lp", "mail", "man", "messagebus", "news", "nobody", "pollinate", "proxy", "root", "sshd", "statd", "sync", "sys", "syslog", "systemd-network", "systemd-resolve", "systemd-timesync", "tcpdump", "tss", "user", "uucp", "uuidd", "www-data"]), linux_users.process(output).unwrap());
    }
}