# MITRE ATT&CK Coverage

This document maps RustHunter's Linux plugins to MITRE ATT&CK techniques, providing visibility into which attack techniques can be detected.

## Persistence (TA0003)

| Technique ID | Technique Name | Plugin(s) | Description |
|--------------|----------------|-----------|-------------|
| T1053.003 | Scheduled Task/Job: Cron | `linux_crontab` | Detects cron jobs that may be used for persistence |
| T1053.006 | Scheduled Task/Job: Systemd Timers | `linux_systemd_timers` | Monitors systemd timer units for scheduled task persistence |
| T1098.004 | Account Manipulation: SSH Authorized Keys | `linux_ssh_keys` | Tracks SSH authorized keys for backdoor access detection |
| T1136.001 | Create Account: Local Account | `linux_users`, `linux_root` | Monitors local user accounts and root-level accounts |
| T1546.004 | Event Triggered Execution: Unix Shell Configuration Modification | `linux_bashrc` | Detects modifications to shell startup scripts |

## Privilege Escalation (TA0004)

| Technique ID | Technique Name | Plugin(s) | Description |
|--------------|----------------|-----------|-------------|
| T1053.003 | Scheduled Task/Job: Cron | `linux_crontab` | Detects cron jobs running with elevated privileges |
| T1053.006 | Scheduled Task/Job: Systemd Timers | `linux_systemd_timers` | Monitors timer units for privilege escalation |
| T1548.001 | Abuse Elevation Control Mechanism: Setuid and Setgid | `linux_suid`, `linux_guid` | Monitors setuid/setgid binaries for exploitation |
| T1574.006 | Hijack Execution Flow: Dynamic Linker Hijacking | `linux_ld_preload` | Detects LD_PRELOAD hijacking for privilege escalation |

## Defense Evasion (TA0005)

| Technique ID | Technique Name | Plugin(s) | Description |
|--------------|----------------|-----------|-------------|
| T1548.001 | Abuse Elevation Control Mechanism: Setuid and Setgid | `linux_suid`, `linux_guid` | Monitors suspicious setuid/setgid binaries |
| T1574.006 | Hijack Execution Flow: Dynamic Linker Hijacking | `linux_ld_preload` | Detects library preloading for hiding activity |

## Discovery (TA0007)

| Technique ID | Technique Name | Plugin(s) | Description |
|--------------|----------------|-----------|-------------|
| T1040 | Network Sniffing | `linux_promisc` | Detects network interfaces in promiscuous mode |
| T1049 | System Network Connections Discovery | `linux_network_connections` | Monitors active TCP/UDP network connections |
| T1057 | Process Discovery | `linux_running_processes` | Lists all running processes with command lines |
| T1082 | System Information Discovery | `linux_kernel_modules` | Enumerates loaded kernel modules |

## Collection (TA0009)

| Technique ID | Technique Name | Plugin(s) | Description |
|--------------|----------------|-----------|-------------|
| T1040 | Network Sniffing | `linux_promisc` | Monitors for network traffic capture capabilities |

## Command and Control (TA0011)

| Technique ID | Technique Name | Plugin(s) | Description |
|--------------|----------------|-----------|-------------|
| T1071.004 | Application Layer Protocol: DNS | `linux_dns` | Monitors DNS server configuration for C2 channels |
| T1571 | Non-Standard Port | `linux_tcp_listen`, `linux_network_connections` | Detects listening services on unusual ports |
| T1572 | Protocol Tunneling | `linux_tcp_listen` | Identifies potential tunneling services |
| T1095 | Non-Application Layer Protocol | `linux_network_connections` | Detects non-standard protocol usage |

## Coverage Summary

### Total Techniques Covered: 18 unique MITRE ATT&CK techniques

### By Tactic:
- **Persistence**: 5 techniques
- **Privilege Escalation**: 4 techniques
- **Defense Evasion**: 2 techniques
- **Discovery**: 4 techniques (+3 new)
- **Collection**: 1 technique
- **Command and Control**: 4 techniques (+1 new)

### Plugin Coverage:
| Plugin | Techniques Covered |
|--------|-------------------|
| `linux_bashrc` | 1 |
| `linux_crontab` | 2 |
| `linux_dns` | 1 |
| `linux_guid` | 2 |
| `linux_kernel_modules` | 2 |
| `linux_ld_preload` | 2 |
| `linux_network_connections` | 3 |
| `linux_promisc` | 1 |
| `linux_root` | 1 |
| `linux_running_processes` | 1 |
| `linux_ssh_keys` | 1 |
| `linux_suid` | 2 |
| `linux_systemd_timers` | 2 |
| `linux_tcp_listen` | 2 |
| `linux_users` | 1 |

### Recent Additions (v0.1.1)
#### Persistence Detection:
- ✅ `linux_ssh_keys` - T1098.004 (SSH Authorized Keys)
- ✅ `linux_systemd_timers` - T1053.006 (Systemd Timers)
- ✅ `linux_bashrc` - T1546.004 (Shell Configuration Modification)
- ✅ `linux_ld_preload` - T1574.006 (Dynamic Linker Hijacking)

#### Discovery Detection:
- ✅ `linux_kernel_modules` - T1082, T1014 (System Information, Rootkit)
- ✅ `linux_network_connections` - T1049, T1571, T1095 (Network Connections Discovery)
- ✅ `linux_running_processes` - T1057 (Process Discovery)

## Future Coverage Plans

### Planned Expansions:
1. **Privilege Escalation**: Sudo configuration, capabilities, polkit rules
2. **Defense Evasion**: Hidden files, rootkit detection, log tampering
3. **Discovery**: Running processes, kernel modules, network connections
4. **Collection**: Clipboard monitoring, screen capture detection
5. **Lateral Movement**: SSH configuration, remote access tools
6. **Exfiltration**: Network connections, scheduled data transfers

## References

- [MITRE ATT&CK for Linux](https://attack.mitre.org/matrices/enterprise/linux/)
- [MITRE ATT&CK Framework](https://attack.mitre.org/)
