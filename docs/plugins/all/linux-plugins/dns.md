# linux_dns

### Description
- Active DNS server(s)


### Configuration
```ini
[linux_dns]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_dns": [
    "8.8.8.8",
    "4.4.4.4",
]
```


### Security Implications
DNS server configuration is critical for network security. Adversaries may manipulate DNS settings to:

- **Data Exfiltration**: Redirect DNS queries to attacker-controlled servers for tunneling data
- **Command and Control**: Use DNS for C2 communication (DNS tunneling)
- **Man-in-the-Middle**: Redirect legitimate traffic to malicious servers
- **Defense Evasion**: Bypass security controls by using unexpected DNS servers
- **Credential Harvesting**: Redirect traffic to phishing sites via DNS poisoning

### Detection Strategy
Monitor for:

- Unexpected changes to DNS server configurations
- Non-standard or suspicious DNS servers (e.g., uncommon public DNS, servers in unusual countries)
- DNS servers in untrusted networks or IP ranges
- Multiple DNS servers when only one is expected
- Removal of legitimate DNS servers

### Notes
!!! note
    - Common legitimate public DNS servers include: Google (8.8.8.8, 8.8.4.4), Cloudflare (1.1.1.1), Quad9 (9.9.9.9)
    - Corporate environments typically use internal DNS servers
    - Changes to DNS configuration should be correlated with authorized change management


### MITRE ATT&CK Mapping
- [T1071.004 Application Layer Protocol: DNS](https://attack.mitre.org/techniques/T1071/004/)
- [T1584.002 Compromise Infrastructure: DNS Server](https://attack.mitre.org/techniques/T1584/002/)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))