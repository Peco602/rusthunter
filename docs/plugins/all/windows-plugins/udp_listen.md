# windows_udp_listen

### Description
- Listening UDP ports


### Notes
!!! note
    - Requires administrator access to get the process names.
    - Likely to provide false positives.


### Configuration
```ini
[windows_udp_listen]
enabled   = false
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | false | Plugin status |


### Returned values
```json
"windows_udp_listen": [
    {
        "LocalAddress":  "::",
        "LocalPort":  123,
        "ProcessName":  "C:\\Windows\\system32\\svchost.exe"
    },
    {
        "LocalAddress":  "::",
        "LocalPort":  500,
        "ProcessName":  "C:\\Windows\\system32\\svchost.exe"
    },
    {
        "LocalAddress":  "::",
        "LocalPort":  4500,
        "ProcessName":  "C:\\Windows\\system32\\svchost.exe"
    }
]
```

| Key | Description |
| --- | ----------- |
| LocalAddress | Listening interface |
| LocalPort | Listening port |
| ProcessName | Name of the process |


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))