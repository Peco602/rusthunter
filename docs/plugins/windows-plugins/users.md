# windows_users

### Description
- Local user accounts


### Configuration
```ini
[windows_users]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"windows_users": [
    {
      "Enabled": false,
      "Name": "Administrator"
    },
    {
      "Enabled": false,
      "Name": "DefaultAccount"
    },
    {
      "Enabled": false,
      "Name": "Guest"
    },
    {
      "Enabled": true,
      "Name": "User"
    }
]
```

| Key | Description |
| --- | ----------- |
| Enabled | User status |
| Name | Username |


### MITRE ATT&CK Mapping
- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))