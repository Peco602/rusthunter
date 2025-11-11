use serde_json::{json, Value};

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxKernelModules {}

impl Plugin for LinuxKernelModules {
    fn name(&self) -> &str {
        "linux_kernel_modules"
    }

    fn description(&self) -> &str {
        "Loaded kernel modules"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "lsmod | tail -n +2";
        match self.execute_command(command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        let mut modules = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let module = json!({
                    "Module": parts[0],
                    "Size": parts[1],
                    "UsedBy": if parts.len() > 3 { parts[3] } else { "-" }
                });
                modules.push(module);
            }
        }

        Ok(json!(modules))
    }
}

impl Default for LinuxKernelModules {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxKernelModules {
    pub fn new() -> Self {
        LinuxKernelModules {}
    }
}

