# linux_bashrc

### Description
- Shell configuration files monitoring
- Detects modifications to shell startup scripts that could be used for persistence or privilege escalation


### Notes
!!! note
    - Monitors both system-wide and per-user shell configuration files
    - System-wide files: `/etc/profile`, `/etc/bash.bashrc`, `/etc/zsh/zshrc`
    - Per-user files: `.bashrc`, `.bash_profile`, `.profile`, `.zshrc` in all home directories
    - Returns MD5 hash of each file for change detection
    - Empty or non-existent files are not reported


### Configuration
```ini
[linux_bashrc]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_bashrc": [
    {
        "User": "system",
        "Filename": "profile",
        "Path": "/etc/profile",
        "MD5": "a1b2c3d4e5f6789012345678901234ab"
    },
    {
        "User": "root",
        "Filename": ".bashrc",
        "Path": "/root/.bashrc",
        "MD5": "b2c3d4e5f67890123456789012345abc"
    },
    {
        "User": "admin",
        "Filename": ".bash_profile",
        "Path": "/home/admin/.bash_profile",
        "MD5": "c3d4e5f678901234567890123456abcd"
    },
    {
        "User": "developer",
        "Filename": ".zshrc",
        "Path": "/home/developer/.zshrc",
        "MD5": "d4e5f6789012345678901234567abcde"
    }
]
```

| Key | Description |
| --- | ----------- |
| User | Username owning the file or "system" for system-wide configurations |
| Filename | Name of the configuration file |
| Path | Full path to the configuration file |
| MD5 | MD5 hash of the file content for change detection |


### Security Implications
Adversaries may modify shell configuration files to maintain persistence by executing malicious code every time a user logs in or opens a shell. Monitoring these files can help detect:

- Backdoor commands executed at shell startup
- Environment variable manipulation for privilege escalation
- Credential harvesting scripts
- Reverse shell connections
- Path hijacking attempts
- Alias poisoning for command substitution


### Detection Strategy
Monitor for:

- Changes to MD5 hashes of shell configuration files
- New files appearing in monitored locations
- Commands or scripts added to shell config files:
  - Network connections (curl, wget, nc, /dev/tcp)
  - Reverse shells or bind shells
  - Base64 encoded commands
  - Downloads from external sources
  - Suspicious export statements (LD_PRELOAD, PATH modifications)
- Unusual aliases (ls, ps, netstat aliased to malicious versions)
- Cron-like functionality in shell configs
- Obfuscated or encoded content

### Baseline Recommendations
- Establish hash baselines for all shell configuration files
- Document legitimate customizations per user
- Review system-wide configs regularly (`/etc/profile`, `/etc/bash.bashrc`)
- Monitor for unauthorized modifications to readonly configs


### MITRE ATT&CK Mapping
- [T1546.004 Event Triggered Execution: Unix Shell Configuration Modification](https://attack.mitre.org/techniques/T1546/004/)


### References
- [Bash Startup Files](https://www.gnu.org/software/bash/manual/html_node/Bash-Startup-Files.html)
- [Zsh Configuration Files](https://zsh.sourceforge.io/Doc/Release/Files.html)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))
