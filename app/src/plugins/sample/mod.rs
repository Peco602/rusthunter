use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct SamplePlugin {}

impl Plugin for SamplePlugin {
    fn name(&self) -> &str {
        &"sample_plugin"  // To be updated
    }

    fn description(&self) -> &str {
        &"Sample description"  // To be updated
    }

    fn os(&self) -> OS {
        OS::Unknown  // To be updated
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "sample command";  // To be updated
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        Ok(()) // To be updated
    }
}

impl SamplePlugin {
    pub fn new() -> Self {
        SamplePlugin {}
    }
}