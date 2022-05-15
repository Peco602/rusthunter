# windows_administrators

## Description
This plugin shows the details about all the autorun entries on a Windows machine.

## Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin activation status |
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

## Associated Files
Path: *launcher/ansible/roles/windows/files*

| File | Description |
| ---- | ----------- |
| *autorunsc64.exe* | [Sysinternals tool](https://docs.microsoft.com/en-us/sysinternals/downloads/autoruns) providing a comprehensive knowledge of auto-starting locations of any startup monitor |

## Return Values
Array of objects:

| Key | Description |
| --- | ----------- |
| Time | Creation time (e.g., 9/19/1943 10:55 AM) |
| Entry Location | Registry key (e.g., HKLM\\System\\CurrentControlSet\\Control\\Session Manager\\BootExecute) |
| Entry | Value (e.g., autocheck autochk *) |
| Enabled | Status (enabled/disabled) |
| Category | Category (e.g., Boot Execute) |
| Profile | Type (e.g., System-wide) |
| Description | Description (e.g., Auto Check Utility) |
| Signer | File signature (e.g., (Verified) Microsoft Windows) |
| Company | File developer (e.g., Microsoft Corporation) |
| Image Path | File location (e.g., c:\\windows\\system32\\autochk.exe) |
| Version | Version (e.g., 10.0.22000.1) |
| Launch String | Command-line (e.g., autocheck autochk *) |
| MD5 | MD5 hash type. |
| SHA-1 | SHA-1 hash type. |
| PESHA-1 | PESHA-1 hash type. |
| PESHA-256 | PESHA-256 hash type. |
| SHA-256 | SHA-256 hash type. |
| IMP | Import hash type. |

## Notes
!!! note
    Requires administrator access to get all the information.

## Authors
- Giovanni Pecoraro (@Peco602)