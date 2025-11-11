use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxLdPreload {}

impl Plugin for LinuxLdPreload {
    fn name(&self) -> &str {
        "linux_ld_preload"
    }

    fn description(&self) -> &str {
        "LD_PRELOAD configurations for library hijacking detection"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        // Check LD_PRELOAD in environment and configuration files
        let command = r#"
            # Check /etc/ld.so.preload
            if [ -f /etc/ld.so.preload ]; then
                while IFS= read -r lib; do
                    [ -z "$lib" ] && continue
                    echo "$lib" | grep -q "^#" && continue
                    echo "ld.so.preload|$lib"
                done < /etc/ld.so.preload
            fi
            
            # Check environment files for LD_PRELOAD
            for env_file in /etc/environment /etc/profile /etc/bash.bashrc /home/*/.bashrc /home/*/.bash_profile /root/.bashrc /root/.bash_profile; do
                if [ -f "$env_file" ]; then
                    grep -h "LD_PRELOAD" "$env_file" 2>/dev/null | grep -v "^#" | while IFS= read -r line; do
                        [ -z "$line" ] && continue
                        source_file=$(basename "$env_file")
                        echo "$source_file|$line"
                    done
                fi
            done
            
            # Check systemd service files
            find /etc/systemd/system /lib/systemd/system /usr/lib/systemd/system -name "*.service" -type f 2>/dev/null | while read service; do
                if grep -q "LD_PRELOAD" "$service" 2>/dev/null; then
                    service_name=$(basename "$service")
                    grep "LD_PRELOAD" "$service" | while IFS= read -r line; do
                        echo "$service_name|$line"
                    done
                fi
            done
        "#;
        match self.execute_command(command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_csv_string_no_header(output, &vec!["Source", "Configuration"], &"|")
    }
}

impl Default for LinuxLdPreload {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxLdPreload {
    pub fn new() -> Self {
        LinuxLdPreload {}
    }
}
