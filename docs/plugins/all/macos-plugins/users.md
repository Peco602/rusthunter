# macos_users

### Description
- Local user accounts


### Configuration
```ini
[macos_users]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"macos_users": [
      "user",
      "daemon",
      "nobody",
      "root"
    ]
```


### MITRE ATT&CK Mapping
- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))