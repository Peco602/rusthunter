# windows_domain_users

### Description
- Domain user and group accounts


### Notes
!!! note
    - Requires a domain account to collect data.
    - Requires the target node to have RSAT tools installed (you can install them via the PowerShell command `Add-WindowsCapability -online -Name "Rsat.ActiveDirectory.DS-LDS.Tools~~~~0.0.1.0")`)
    - It is suggested to run this all domain plugins on a single machine to avoid repeated results.


### Configuration
```ini
[windows_domain_users]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"windows_domain_users": [
  {
      "Name":  "Administrator",
      "ObjectClass":  "user",
      "Enabled":  true
  },
  {
      "Name":  "Guest",
      "ObjectClass":  "user",
      "Enabled":  false
  },
  {
      "Name":  "krbtgt",
      "ObjectClass":  "user",
      "Enabled":  false
  }
]
```

| Key | Description |
| --- | ----------- |
| Name | Username |
| ObjectClass | Account type (User/Group) |
| Enabled | Account status |



### MITRE ATT&CK Mapping
- [T1136.002 Create Account: Domain Account](https://attack.mitre.org/techniques/T1136/002/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))