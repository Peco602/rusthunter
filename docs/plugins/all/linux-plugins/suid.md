# linux_suid

### Description
- Files with Setuid bit enabled


### Configuration
```ini
[linux_suid]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_suid": [
    "/usr/lib/snapd/snap-confine",
    "/usr/lib/xorg/Xorg.wrap",
    "/usr/lib/openssh/ssh-keysign",
    "/usr/lib/dbus-1.0/dbus-daemon-launch-helper",
    "/usr/sbin/pppd",
    "/usr/libexec/polkit-agent-helper-1",
    "/usr/bin/chfn",
    "/usr/bin/umount",
    "/usr/bin/mount",
    "/usr/bin/vmware-user-suid-wrapper",
    "/usr/bin/newgrp",
    "/usr/bin/sudo",
    "/usr/bin/gpasswd",
    "/usr/bin/pkexec",
    "/usr/bin/su",
    "/usr/bin/chsh",
    "/usr/bin/fusermount",
    "/usr/bin/passwd"
]
```


### Security Implications
The setuid (Set User ID) bit allows executables to run with the privileges of the file's owner, typically root. Adversaries may exploit or create setuid binaries to:

- **Privilege Escalation**: Execute code with root or elevated user permissions
- **Persistence**: Maintain privileged access through backdoored setuid binaries
- **Defense Evasion**: Hide malicious activity within legitimate-looking binaries
- **Credential Access**: Dump credentials using elevated privileges

Monitoring setuid files can help detect:

- Unauthorized setuid binaries in unusual locations (e.g., `/tmp`, `/dev/shm`)
- Modified system binaries with setuid bit set
- Setuid files in world-writable directories
- Custom executables with unnecessary setuid root privileges
- Known vulnerable setuid binaries that could be exploited


### Detection Strategy
Monitor for:

- New setuid files appearing in snapshots
- Setuid binaries in high-risk locations:
  - `/tmp`, `/dev/shm`, `/var/tmp` (world-writable directories)
  - User home directories (`/home/*`, `/root`)
  - Web server directories (`/var/www`, `/usr/share/nginx`)
- Changes to existing system binaries (unexpected setuid bit)
- Setuid files owned by root in non-standard locations
- Executables with both setuid and world-writable permissions (extreme risk)
- Setuid shell scripts (rarely legitimate, often malicious)
- Known vulnerable binaries listed on [GTFOBins](https://gtfobins.github.io/)

### Baseline Recommendations
- Maintain a whitelist of legitimate setuid binaries
- Compare against baseline from fresh OS installation
- Investigate any setuid binary not in `/usr/bin`, `/usr/sbin`, `/bin`, `/sbin`
- Use file integrity monitoring for critical system binaries


### MITRE ATT&CK Mapping
- [T1548.001 Abuse Elevation Control Mechanism: Setuid and Setgid](https://attack.mitre.org/techniques/T1548/001/)


### References
- [GTFOBins](https://gtfobins.github.io/) - Curated list of Unix binaries that can be exploited for privilege escalation


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))