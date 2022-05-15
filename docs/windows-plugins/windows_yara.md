# windows_yara

## Description
This plugin executes a customizable Yara rule scan over the file system of a Windows machine.

## Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin activation status |
| scan_path | Windows path | c:\ | Plugin activation status |

## Associated Files
Path: *launcher/ansible/roles/windows/files*

| File | Description |
| ---- | ----------- |
| *yara64.exe* | [Yara](https://github.com/VirusTotal/yara) 64-bit Windows executable. |
| *yara.yml* | Customizable Yara rule file. |

## Return Values
Array of strings:
- *"ExampleRule C:\Path\file1.txt*
- *"ExampleRule C:\Path\file2.txt*

## Notes
!!! note
    Requires administrator access to scan all possible paths.

## Authors
- Giovanni Pecoraro (@Peco602)