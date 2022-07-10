use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxHowManyCronjobs {}

impl Plugin for LinuxHowManyCronjobs {
    fn name(&self) -> &str {
        &"how_many_cronjobs"
    }

    fn description(&self) -> &str {
        &"Get the number of active cronjobs"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "crontab -l 2>/dev/null | sed 's/^ *//;/^[*@0-9]/!d' | wc -l";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxHowManyCronjobs {
    pub fn new() -> Self {
        LinuxHowManyCronjobs {}
    }
}