# windows_administrators

### Description
- Local administrator accounts


### Configuration
```ini
[windows_administrators]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"windows_administrators": [
    {
      "Name": "HOST\\Administrator",
      "ObjectClass": "User"
    },
    {
      "Name": "HOST\\User",
      "ObjectClass": "User"
    }
]
```

| Key | Description |
| --- | ----------- |
| Name | Username |
| ObjectClass | Account type (User/Group) |


### MITRE ATT&CK Mapping
- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))