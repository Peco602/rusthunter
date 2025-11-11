# linux_guid

### Description
- Files with Setgid bit enabled


### Configuration
```ini
[linux_guid]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_guid": [
    "/usr/lib/xorg/Xorg.wrap",
    "/usr/local/share/fonts",
    "/usr/sbin/pam_extrausers_chkpwd",
    "/usr/sbin/unix_chkpwd",
    "/usr/libexec/camel-lock-helper-1.2",
    "/usr/share/ppd/custom",
    "/usr/bin/expiry",
    "/usr/bin/write.ul",
    "/usr/bin/crontab",
    "/usr/bin/chage",
    "/usr/bin/wall",
    "/usr/bin/ssh-agent"
]
```


### Security Implications
The setgid (Set Group ID) bit allows executables to run with the privileges of the file's group owner. Adversaries may exploit or create setgid binaries to:

- **Privilege Escalation**: Execute code with elevated group permissions
- **Persistence**: Maintain access through modified setgid binaries
- **Defense Evasion**: Hide malicious activity within legitimate-looking binaries
- **Lateral Movement**: Access resources restricted to specific groups

Monitoring setgid files can help detect:

- Unauthorized setgid binaries in unusual locations
- Modified system binaries with setgid bit set
- Setgid files in world-writable directories
- Custom executables with unnecessary setgid privileges


### Detection Strategy
Monitor for:

- New setgid files appearing in snapshots
- Setgid binaries in suspicious locations:
  - `/tmp`, `/dev/shm`, `/var/tmp` (world-writable)
  - User home directories
  - Web server directories (`/var/www`, `/usr/share/nginx`)
- Changes to existing system binaries (modified setgid bit)
- Setgid files owned by unexpected groups
- Executables with both setgid and world-writable permissions
- Setgid scripts (shell scripts with setgid should be investigated)

### Baseline Recommendations
- Create a whitelist of known legitimate setgid binaries
- Compare against standard system installations
- Review setgid files in non-standard locations carefully
- Check file ownership and group context


### MITRE ATT&CK Mapping
- [T1548.001 Abuse Elevation Control Mechanism: Setuid and Setgid](https://attack.mitre.org/techniques/T1548/001/)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))