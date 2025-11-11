# linux_root

### Description
- Local root accounts


### Configuration
```ini
[linux_root]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_root": [
    "root"
]
```


### Security Implications
Accounts with UID 0 have full system privileges (root access). Adversaries may create additional root accounts to:

- **Persistence**: Maintain privileged access even if the primary root account is secured
- **Privilege Escalation**: Gain full system control
- **Defense Evasion**: Hide privileged access in accounts that appear less suspicious
- **Backdoor Access**: Create hidden administrative accounts for later use

### Detection Strategy
Monitor for:

- Multiple accounts with UID 0 (typically only "root" should exist)
- New accounts with UID 0 appearing in snapshots
- Accounts with suspicious names but root privileges
- Changes to existing account UIDs to 0

### Notes
!!! warning
    - In a healthy system, only the "root" account should have UID 0
    - Any additional accounts with UID 0 are highly suspicious and warrant immediate investigation
    - Some systems may have "toor" (root backwards) as a legitimate backup root account, but this is rare


### MITRE ATT&CK Mapping
- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)
- [T1078.003 Valid Accounts: Local Accounts](https://attack.mitre.org/techniques/T1078/003/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))