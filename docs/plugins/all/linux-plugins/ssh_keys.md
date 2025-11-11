# linux_ssh_keys

### Description
- SSH authorized keys for all users
- Monitors SSH public key authentication configurations that could be used for persistence


### Notes
!!! note
    - Requires administrator access to read authorized_keys files from all user home directories
    - Monitors both `/root/.ssh/authorized_keys` and `/home/*/.ssh/authorized_keys`
    - Empty or non-existent authorized_keys files are not reported


### Configuration
```ini
[linux_ssh_keys]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_ssh_keys": [
    {
        "User": "root",
        "KeyType": "ssh-rsa",
        "Comment": "user@workstation"
    },
    {
        "User": "admin",
        "KeyType": "ssh-ed25519",
        "Comment": "admin@server"
    },
    {
        "User": "developer",
        "KeyType": "ecdsa-sha2-nistp256",
        "Comment": "dev@laptop"
    }
]
```

| Key | Description |
| --- | ----------- |
| User | Username owning the authorized_keys file |
| KeyType | SSH key algorithm (ssh-rsa, ssh-ed25519, ecdsa-sha2-nistp256, etc.) |
| Comment | Comment/identifier associated with the public key |


### Security Implications
Adversaries may add SSH authorized keys to maintain persistent access to compromised systems. Monitoring changes to authorized_keys files can help detect:

- Unauthorized SSH key additions
- Backdoor accounts with unexpected SSH access
- Compromised user accounts with new SSH keys
- Lateral movement attempts via SSH key-based authentication


### Detection Strategy
Monitor for:

- New SSH keys appearing in authorized_keys files
- Keys added to service accounts that shouldn't have SSH access
- Keys with generic or suspicious comments (or no comment)
- Multiple keys added simultaneously across different accounts
- Keys from unknown sources or unexpected key types
- Duplicate keys across multiple user accounts
- Keys in accounts that rarely use SSH authentication
- Weak key algorithms (RSA < 2048 bits, DSA)

### Baseline Recommendations
- Document authorized SSH keys for each user and service account
- Maintain inventory of legitimate key fingerprints
- Enforce key comment policies (should identify owner/purpose)
- Regular audit of authorized_keys across all systems
- Remove keys for departed personnel


### MITRE ATT&CK Mapping
- [T1098.004 Account Manipulation: SSH Authorized Keys](https://attack.mitre.org/techniques/T1098/004/)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))
