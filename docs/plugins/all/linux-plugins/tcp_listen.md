# linux_tcp_listen

### Description
- Listening TCP ports


### Notes
!!! note
    Requires administrator access to get the process names.


### Configuration
```ini
[linux_tcp_listen]
enabled   = false
```


### Returned values
```json
"linux_tcp_listen": [
    {
        "Port": "127.0.0.1:631",
        "Process": "cupsd",
        "User": "root"
    },
    {
        "Port": "[::1]:631",
        "Process": "cupsd",
        "User": "root"
    },
    {
        "Port": "*:22",
        "Process": "sshd",
        "User": "root"
    },
    {
        "Port": "*:22",
        "Process": "sshd",
        "User": "root"
    },
    {
        "Port": "127.0.0.53:53",
        "Process": "systemd-r",
        "User": "systemd-resolve"
    }
]
```

| Key | Description |
| --- | ----------- |
| Process | Name of the process |
| Port | Listening port on a specific interface  |
| User | User running the process |


### Security Implications
Listening TCP ports can indicate services running on the system. Adversaries may:

- **Backdoor Installation**: Open listening ports for remote access (reverse shells, bind shells)
- **Command and Control**: Establish C2 channels through unexpected listening services
- **Lateral Movement**: Deploy remote access tools or tunneling services
- **Defense Evasion**: Use non-standard ports or disguise malicious services as legitimate ones

### Detection Strategy
Monitor for:

- New listening ports that weren't present in previous snapshots
- Processes listening on unusual or non-standard ports
- Legitimate processes listening on unexpected ports
- Services bound to external interfaces (0.0.0.0 or *) instead of localhost
- High-numbered ports (ephemeral range) with persistent listeners


### MITRE ATT&CK Mapping
- [T1572 Protocol Tunneling](https://attack.mitre.org/techniques/T1572/)
- [T1571 Non-Standard Port](https://attack.mitre.org/techniques/T1571/)
- [T1219 Remote Access Software](https://attack.mitre.org/techniques/T1219/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))