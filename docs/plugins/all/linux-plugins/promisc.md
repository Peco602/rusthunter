# linux_promisc

### Description
- Network interface in promiscuous mode


### Configuration
```ini
[linux_promisc]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_promisc": [
    "eth0"
]
```


### Security Implications
Network interfaces in promiscuous mode can capture all network traffic passing through the interface, not just traffic destined for the system. Adversaries may enable promiscuous mode to:

- **Network Sniffing**: Capture sensitive data (credentials, session tokens, confidential information)
- **Reconnaissance**: Monitor network traffic to map the network and identify targets
- **Credential Harvesting**: Intercept authentication traffic (HTTP, FTP, Telnet, etc.)
- **Man-in-the-Middle Attacks**: Facilitate traffic interception and manipulation

### Detection Strategy
Monitor for:

- Network interfaces unexpectedly in promiscuous mode
- Interfaces that should never be in promiscuous mode (production servers)
- Correlation with packet capture tools (tcpdump, wireshark, tshark) running on the system
- Promiscuous mode enabled on non-monitoring systems

### Notes
!!! note
    - Some legitimate uses include network monitoring, IDS/IPS systems, and packet capture for troubleshooting
    - Virtual machine network adapters may legitimately be in promiscuous mode for bridged networking


### MITRE ATT&CK Mapping
- [T1040 Network Sniffing](https://attack.mitre.org/techniques/T1040/)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))