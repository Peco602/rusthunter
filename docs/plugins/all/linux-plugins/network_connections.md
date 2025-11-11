# linux_network_connections

### Description
- Active network connections (TCP and UDP)
- Monitors established and listening network connections for C2 and lateral movement detection


### Notes
!!! note
    - Uses `ss` command (modern replacement for netstat)
    - Shows both TCP and UDP connections
    - Includes process information when available (requires root privileges)
    - Lists all connection states (ESTABLISHED, LISTEN, TIME_WAIT, etc.)


### Configuration
```ini
[linux_network_connections]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_network_connections": [
    {
        "Protocol": "tcp",
        "State": "ESTAB",
        "LocalAddress": "192.168.1.100:52342",
        "RemoteAddress": "93.184.216.34:443",
        "Process": "users:((\"firefox\",pid=2847,fd=89))"
    },
    {
        "Protocol": "tcp",
        "State": "LISTEN",
        "LocalAddress": "0.0.0.0:22",
        "RemoteAddress": "0.0.0.0:*",
        "Process": "users:((\"sshd\",pid=1024,fd=3))"
    },
    {
        "Protocol": "udp",
        "State": "UNCONN",
        "LocalAddress": "127.0.0.53:53",
        "RemoteAddress": "0.0.0.0:*",
        "Process": "users:((\"systemd-resolve\",pid=712,fd=12))"
    }
]
```

| Key | Description |
| --- | ----------- |
| Protocol | Network protocol (tcp, udp) |
| State | Connection state (ESTAB, LISTEN, UNCONN, etc.) |
| LocalAddress | Local IP address and port |
| RemoteAddress | Remote IP address and port (or 0.0.0.0:* for listening) |
| Process | Process name, PID, and file descriptor (if available) |


### Security Implications
Network connections are critical indicators of adversary activity. Monitoring active connections can detect:

- **Command and Control**: Connections to known malicious IPs or suspicious domains
- **Data Exfiltration**: Large data transfers to external destinations
- **Lateral Movement**: Connections to internal systems via RDP, SSH, SMB
- **Remote Access Tools**: Backdoors and reverse shells
- **Port Scanning**: Multiple connections to sequential ports
- **Beaconing**: Periodic connections to external systems

### Detection Strategy
Monitor for:

- Connections to unusual or suspicious IP addresses/ports
- Processes making unexpected network connections
- High-numbered (ephemeral) ports with persistent connections
- Connections to known malicious infrastructure
- Internal network scanning patterns
- Unusual protocols or ports for specific processes
- Connections from system processes that shouldn't communicate externally

### Baseline Recommendations
Establish baselines for:

- Normal outbound connection destinations
- Expected listening services and ports
- Typical network connection patterns per process
- Authorized remote access patterns


### MITRE ATT&CK Mapping
- [T1049 System Network Connections Discovery](https://attack.mitre.org/techniques/T1049/)
- [T1571 Non-Standard Port](https://attack.mitre.org/techniques/T1571/)
- [T1095 Non-Application Layer Protocol](https://attack.mitre.org/techniques/T1095/)


### References
- [ss command documentation](https://man7.org/linux/man-pages/man8/ss.8.html)
- [Network Connection Analysis](https://www.sans.org/blog/finding-hidden-threats-by-monitoring-network-connections/)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))
