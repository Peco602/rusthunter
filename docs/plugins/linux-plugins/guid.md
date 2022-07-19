# linux_guid

### Description
This plugin provides a list of the files with the setgid bit enabled on a Linux machine. 

An adversary may abuse configurations where an application has the setuid or setgid bits set in order to get code running in a different (and possibly more privileged) user’s context. On Linux or macOS, when the setuid or setgid bits are set for an application binary, the application will run with the privileges of the owning user or group respectively.

### Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |

### Returned values
Array of files with setguid bit enabled:

- *file 1*
- *file 2*

### MITRE ATT&CK Mapping
- [T1548.001 Abuse Elevation Control Mechanism: Setuid and Setgid](https://attack.mitre.org/techniques/T1548/001/)

### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))