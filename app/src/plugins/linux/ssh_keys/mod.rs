use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxSshKeys {}

impl Plugin for LinuxSshKeys {
    fn name(&self) -> &str {
        "linux_ssh_keys"
    }

    fn description(&self) -> &str {
        "SSH authorized keys for all users"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        // Get all users with home directories and check their authorized_keys
        let command = r#"
            for user_home in /home/* /root; do
                if [ -d "$user_home" ] && [ -f "$user_home/.ssh/authorized_keys" ]; then
                    user=$(basename "$user_home")
                    [ "$user_home" = "/root" ] && user="root"
                    while IFS= read -r key; do
                        # Skip empty lines and comments
                        [ -z "$key" ] && continue
                        echo "$key" | grep -q "^#" && continue
                        # Extract key type and fingerprint
                        key_type=$(echo "$key" | awk '{print $1}')
                        key_comment=$(echo "$key" | awk '{print $NF}')
                        echo "$user|$key_type|$key_comment"
                    done < "$user_home/.ssh/authorized_keys"
                fi
            done
        "#;
        match self.execute_command(command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_csv_string_no_header(
            output,
            &vec!["User", "KeyType", "Comment"],
            &"|",
        )
    }
}

impl Default for LinuxSshKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxSshKeys {
    pub fn new() -> Self {
        LinuxSshKeys {}
    }
}
