use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};
use crate::validator::validate_windows_sam_account_name;

pub struct WindowsDomainGroup {}

impl Plugin for WindowsDomainGroup {
    fn name(&self) -> &str {
        &"windows_domain_group"
    }

    fn description(&self) -> &str {
        &"Domain group members"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let group_name =  match _config.get_string_setting(self.name(), &"group_name") {
            Some(v) => { if validate_windows_sam_account_name(&v) {
                    v
                } else {
                    return Err(format!("Not valid group_name setting: {}", v));
                }}, 
            None => "Domain Admins".to_string(),
        };
        let command = format!("Get-ADGroupMember -Identity \"{}\" | Select-Object Name,ObjectClass | Sort-Object -Property Name | ForEach-Object {{ ConvertTo-Json @($_) }}", group_name);
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsDomainGroup {
    pub fn new() -> Self {
        WindowsDomainGroup {}
    }
}