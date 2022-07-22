# windows_domain_computers

### Description
- Domain computers


### Notes
!!! note
    - Requires a domain account to collect data.
    - Requires the target node to have RSAT tools installed (you can install them via the PowerShell command `Add-WindowsCapability -online -Name "Rsat.ActiveDirectory.DS-LDS.Tools~~~~0.0.1.0")`)
    - It is suggested to run this all domain plugins on a single machine to avoid repeated results (use an hosts file with a single windows domain target node).


### Configuration
```ini
[windows_domain_computers]
enabled = false
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | false | Plugin status |


### Returned values
```json
"windows_domain_computers": [
  {
    "Enabled":  true,
    "Name":  "DC"
  },
  {
    "Enabled":  true,
    "Name":  "WS01",
  }
],
```

| Key | Description |
| --- | ----------- |
| Enabled | Account status |
| Name | Computer name |


### MITRE ATT&CK Mapping
- [T1136.002 Create Account: Domain Account](https://attack.mitre.org/techniques/T1136/002/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))