# windows_tcp_listen

### Description
- Listening TCP ports


### Configuration
```ini
[windows_tcp_listen]
enabled   = false
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | false | Plugin status |


### Returned values
```json
"windows_tcp_listen": [
    {
        "LocalAddress":  "::",
        "LocalPort":  135,
        "ProcessName":  "C:\\Windows\\system32\\svchost.exe"
    },
    {
        "LocalAddress":  "::",
        "LocalPort":  443,
        "ProcessName":  "C:\\Program Files (x86)\\VMware\\VMware Workstation\\vmware-hostd.exe"
    },
    {
        "LocalAddress":  "::",
        "LocalPort":  445,
        "ProcessName":  null
    },
    {
        "LocalAddress":  "::",
        "LocalPort":  17500,
        "ProcessName":  "C:\\Program Files (x86)\\Dropbox\\Client\\Dropbox.exe"
    }
]
```

| Key | Description |
| --- | ----------- |
| LocalAddress | Listening interface |
| LocalPort | Listening port |
| ProcessName | Name of the process |


### Notes
!!! note
    Requires administrator access to get the process name.


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))