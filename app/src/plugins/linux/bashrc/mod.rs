use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxBashrc {}

impl Plugin for LinuxBashrc {
    fn name(&self) -> &str {
        "linux_bashrc"
    }

    fn description(&self) -> &str {
        "Shell configuration files for persistence detection"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        // Check common shell config files that can be used for persistence
        let command = r#"
            for config_file in /etc/profile /etc/bash.bashrc /etc/zsh/zshrc /home/*/.bashrc /home/*/.bash_profile /home/*/.profile /home/*/.zshrc /root/.bashrc /root/.bash_profile /root/.profile /root/.zshrc; do
                if [ -f "$config_file" ]; then
                    user=$(echo "$config_file" | cut -d'/' -f3)
                    [ "$config_file" = "/etc/profile" ] || [ "$config_file" = "/etc/bash.bashrc" ] || [ "$config_file" = "/etc/zsh/zshrc" ] && user="system"
                    [ "$(echo "$config_file" | cut -d'/' -f2)" = "root" ] && user="root"
                    filename=$(basename "$config_file")
                    md5sum=$(md5sum "$config_file" | awk '{print $1}')
                    echo "$user|$filename|$config_file|$md5sum"
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
            &vec!["User", "Filename", "Path", "MD5"],
            &"|",
        )
    }
}

impl Default for LinuxBashrc {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxBashrc {
    pub fn new() -> Self {
        LinuxBashrc {}
    }
}
