use serde_json::Value;

use crate::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxTCPListen {}

impl Plugin for LinuxTCPListen {
    fn name(&self) -> &str {
        &"linux_tcp_listen"
    }

    fn description(&self) -> &str {
        &"List of users"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "lsof -nP -iTCP -sTCP:LISTEN | grep -v COMMAND | tr -s ' ' |  cut -d ' ' -f 1,3,9 | sort";
        match self.linux_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_csv_string_no_header(output, &vec!["Process", "User", "Port"], &" ")
    }
}

impl LinuxTCPListen {
    pub fn new() -> Self {
        LinuxTCPListen {}
    }
}

#[cfg(test)]
mod tests {
    use std::include_str;
    use serde_json::json;
    use super::*;

    #[test]
    fn linux_tcp_listen() {
        let data = json!([
            {
                "Process": "cupsd",
                "Port": "127.0.0.1:631",
                "User": "root"
            },
            {
                "Process": "cupsd",
                "Port": "[::1]:631",
                "User": "root"
            },
            {
                "Process": "nc",
                "Port": "*:10000",
                "User": "user"
            },
            {
                "Process": "sshd",
                "Port": "*:22",
                "User": "root"
            },
            {
                "Process": "sshd",
                "Port": "*:22",
                "User": "root"
            },
            {
                "Process": "systemd-r",
                "Port": "127.0.0.53:53",
                "User": "systemd-resolve"
            }
        ]);
        let output = include_str!("output.txt");
        let linux_tcp_listen = LinuxTCPListen::new();
        assert_eq!(data, linux_tcp_listen.process(output).unwrap());
    }
}