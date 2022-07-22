# windows_yara

### Description
- Custom YARA rule scanning


### Notes
!!! note
    Requires administrator access to scan all paths.


### Requirements
The following files are required:

| File | Description |
| ---- | ----------- |
| *yara64.exe* | [YARA](https://github.com/VirusTotal/yara) 64-bit Windows executable. |
| *yara.yml* | Customizable YARA rule file. |

in the path "*launcher/ansible/roles/windows/files*".


### Configuration
```ini
[windows_yara]
enabled   = false
scan_path = c:\
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | false | Plugin status |
| scan_path | Windows path | c:\ | YARA scanning path |


### Returned values
```json
"windows_yara": [
    "ExampleRule C:\Path\file1.txt",
    "ExampleRule C:\Path\file2.txt"
]
```


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))
