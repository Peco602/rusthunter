use serde_json::{Value};

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsAutoruns {}

impl Plugin for WindowsAutoruns {
    fn name(&self) -> &str {
        &"windows_autoruns"
    }

    fn description(&self) -> &str {
        &"Autorun entries"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let mut command = format!("{}\\{}", _binary_directory, "autorunsc64.exe /accepteula -a ");
        if _config.get_boolean_setting(self.name(), "boot_execute") {
            command.push_str("b");
        }
        if _config.get_boolean_setting(self.name(), "appinit_dlls") {
            command.push_str("d");
        }
        if _config.get_boolean_setting(self.name(), "explorer_addons") {
            command.push_str("e");
        }
        if _config.get_boolean_setting(self.name(), "sidebar_gadgets") {
            command.push_str("g");
        }
        if _config.get_boolean_setting(self.name(), "image_hijacks") {
            command.push_str("h");
        }
        if _config.get_boolean_setting(self.name(), "ie_addons") {
            command.push_str("i");
        }
        if _config.get_boolean_setting(self.name(), "known_dlls") {
            command.push_str("k");
        }
        if _config.get_boolean_setting(self.name(), "logon_startups") {
            command.push_str("l");
        }
        if _config.get_boolean_setting(self.name(), "wmi_entries") {
            command.push_str("m");
        }
        if _config.get_boolean_setting(self.name(), "winsock_protocol") {
            command.push_str("n");
        }
        if _config.get_boolean_setting(self.name(), "codecs") {
            command.push_str("o");
        }      
        if _config.get_boolean_setting(self.name(), "printer_dlls") {
            command.push_str("p");
        }
        if _config.get_boolean_setting(self.name(), "lsa_providers") {
            command.push_str("r");
        }
        if _config.get_boolean_setting(self.name(), "autostart_services") {
            command.push_str("s");
        }
        if _config.get_boolean_setting(self.name(), "scheduled_tasks") {
            command.push_str("t");
        }
        if _config.get_boolean_setting(self.name(), "winlogon_entries") {
            command.push_str("w");
        }
        // http://brianvanderplaats.com/2015/10/08/generating-json-from-csv-using-powershell/
        command.push_str(" -c -h -s -nobanner 2> $null | ConvertFrom-Csv | Select-Object Category,Enabled,Entry,'Image Path','Launch String',MD5,SHA-1,Signer,Time | Sort-Object -Property Category,Entry | ConvertTo-Json");

        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsAutoruns {
    pub fn new() -> Self {
        WindowsAutoruns {}
    }
}