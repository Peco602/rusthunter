# windows_administrators

### Description
- Autorun entries


### Requirements
The following file is required:

| File | Description |
| ---- | ----------- |
| *autorunsc64.exe* | [Sysinternals tool](https://docs.microsoft.com/en-us/sysinternals/downloads/autoruns) providing a comprehensive knowledge of auto-starting locations of any startup monitor |

in the path "*launcher/ansible/roles/windows/files*".


### Configuration
```ini
[windows_autoruns]
enabled             = true
boot_execute        = true
appinit_dlls        = false
explorer_addons     = false
sidebar_gadgets     = false
image_hijacks       = false
ie_addons           = false
known_dlls          = false
logon_startups      = true
wmi_entries         = true
winsock_protocol    = false
codecs              = false
printer_dlls        = false
lsa_providers       = false
autostart_services  = true
scheduled_tasks     = true
winlogon_entries    = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |
| boot_execute | true/false | true | Boot execute. |
| appinit_dlls | true/false | false | Appinit DLLs. |
| explorer_addons | true/false | false | Explorer addons. |
| sidebar_gadgets | true/false | false | Sidebar gadgets (Vista and higher). |
| image_hijacks | true/false | false | Image hijacks. |
| ie_addons | true/false | false | Internet Explorer addons. |
| known_dlls | true/false | false | Known DLLs. |
| logon_startups | true/false | true | Logon startups (this is the default). |
| wmi_entries | true/false | true | WMI entries. |
| winsock_protocol | true/false | false | Winsock protocol and network providers. |
| codecs | true/false | false | Codecs. |
| printer_dlls | true/false | false | Printer monitor DLLs. |
| lsa_providers | true/false | false | LSA security providers. |
| autostart_services | true/false | true | Autostart services and non-disabled drivers. |
| scheduled_tasks | true/false | true | Scheduled tasks. |
| winlogon_entries | true/false | true | Winlogon entries. |


### Returned values
```json
"windows_autoruns": [
    {
        "Category":  "Known DLLs",
        "Enabled":  "enabled",
        "Entry":  "kernel32",
        "Image Path":  "c:\\windows\\system32\\kernel32.dll",
        "Launch String":  "kernel32.dll",
        "MD5":  "E2143783F3A526E29BAE7F43E4FE301C",
        "SHA-1":  "6CE38356568C7DEE9556DF557C2F8683E0D1A1AF",
        "Signer":  "(Verified) Microsoft Windows",
        "Time":  "02/03/2011 00:27"
    },
    {
        "Category":  "Logon",
        "Enabled":  "enabled",
        "Entry":  "explorer.exe",
        "Image Path":  "c:\\windows\\explorer.exe",
        "Launch String":  "explorer.exe",
        "MD5":  "D8F2FFB4A0842831337778C1A5E4FFD5",
        "SHA-1":  "4544F7534D80CB368C4979BFE7E570D8EC0834D9",
        "Signer":  "(Verified) Microsoft Windows",
        "Time":  "19/01/1919 05:36"
    }
]
```

| Key | Description |
| --- | ----------- |
| Time | Creation time (e.g., 9/19/1943 10:55 AM) |
| Entry | Value (e.g., autocheck autochk *) |
| Enabled | Status (enabled/disabled) |
| Category | Category (e.g., Boot Execute) |
| Signer | File signature (e.g., (Verified) Microsoft Windows) |
| Image Path | File location (e.g., c:\\windows\\system32\\autochk.exe) |
| Launch String | Command-line (e.g., autocheck autochk *) |
| MD5 | MD5 hash type. |
| SHA-1 | SHA-1 hash type. |
<!-- | Entry Location | Registry key (e.g., HKLM\\System\\CurrentControlSet\\Control\\Session Manager\\BootExecute) | -->
<!-- | Profile | Type (e.g., System-wide) | -->
<!-- | Description | Description (e.g., Auto Check Utility) | -->
<!-- | Company | File developer (e.g., Microsoft Corporation) | -->
<!-- | Version | Version (e.g., 10.0.22000.1) | -->
<!-- | PESHA-1 | PESHA-1 hash type. | -->
<!-- | PESHA-256 | PESHA-256 hash type. | -->
<!-- | SHA-256 | SHA-256 hash type. | -->
<!-- | IMP | Import hash type. | -->


### Notes
!!! note
    Requires administrator access to get full data.


### MITRE ATT&CK Mapping
- [T1037 Boot or Logon Initialization Scripts](https://attack.mitre.org/techniques/T1037/)
- [T1547 Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547/)
- [T1176 Browser Extensions](https://attack.mitre.org/techniques/T1176/)
- [T1547.008 Boot or Logon Autostart Execution: LSASS Driver](https://attack.mitre.org/techniques/T1547/008/)
- [T1547.001 Boot or Logon Autostart Execution: Registry Run Keys / Startup Folder](https://attack.mitre.org/techniques/T1547/001/)
- [T1547.002 Boot or Logon Autostart Execution: Authentication Package](https://attack.mitre.org/techniques/T1547/002/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))