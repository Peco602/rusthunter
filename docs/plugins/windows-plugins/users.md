# windows_users

### Description
This plugin provides a list of the available users on a Windows machine. Adversaries may create a local account to maintain access to victim systems. Local accounts are those configured by an organization for use by users, remote support, services, or for administration on a single system or service.

### Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |

### Returned values
Array of objects:

| Key | Description |
| --- | ----------- |
| Name | Username |
| Enabled | Status |

### MITRE ATT&CK Mapping

- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)

### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))