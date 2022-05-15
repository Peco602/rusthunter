use serde_json::{Value};

use crate::Config;
use crate::plugins::{Plugin, OS};

pub struct WindowsAutoruns {}

impl Plugin for WindowsAutoruns {
    fn name(&self) -> &str {
        &"windows_autoruns"
    }

    fn description(&self) -> &str {
        &"List of autorun entries"
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
        command.push_str(" -c -h -s -nobanner 2> $null | ConvertFrom-Csv | ConvertTo-Json");

        match self.windows_powershell_command(&command) {
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

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::include_str;
    use super::*;

    #[test]
    fn windows_autoruns() {
        let data = json!([
            {
                "Time":  "3/10/2022 12:19 AM",
                "Entry Location":  "HKLM\\System\\CurrentControlSet\\Control\\Session Manager\\BootExecute",
                "Entry":  "",
                "Enabled":  "",
                "Category":  "Boot Execute",
                "Profile":  "System-wide",
                "Description":  "",
                "Signer":  "",
                "Company":  "",
                "Image Path":  "",
                "Version":  "",
                "Launch String":  "",
                "MD5":  "",
                "SHA-1":  "",
                "PESHA-1":  "",
                "PESHA-256":  "",
                "SHA-256":  null,
                "IMP":  null
            },
            {
            "Time":  "9/19/1943 10:55 AM",
            "Entry Location":  "HKLM\\System\\CurrentControlSet\\Control\\Session Manager\\BootExecute",
            "Entry":  "autocheck autochk *",
            "Enabled":  "enabled",
            "Category":  "Boot Execute",
            "Profile":  "System-wide",
            "Description":  "Auto Check Utility",
            "Signer":  "(Verified) Microsoft Windows",
            "Company":  "Microsoft Corporation",
            "Image Path":  "c:\\windows\\system32\\autochk.exe",
            "Version":  "10.0.22000.1",
            "Launch String":  "autocheck autochk *",
            "MD5":  "5EA251B631E3CEFCD1411EAFBDB12BE1",
            "SHA-1":  "50D66CA64912BE86A1159AB65E7BAFCB27D5F381",
            "PESHA-1":  "04044923CCCF14FF20A14A0E5B40E868215227E6",
            "PESHA-256":  "38500AA7E903B9F083B4EF089CCF81CF1DC5FABBA2CCD09CCDA566242C61C3F4",
            "SHA-256":  "AE3F9FDFD44E203F9A8DC96F29D165407573E0853E8CCEE81AF3D7B460A3A785",
            "IMP":  "020B9CFBEF6C56682225F237706926B0"
        }]);
        let output = include_str!("output.txt");
        let windows_autoruns = WindowsAutoruns::new();
        assert_eq!(data, windows_autoruns.process(output).unwrap());
    }
}