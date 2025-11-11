# linux_ld_preload

### Description
- LD_PRELOAD configurations monitoring
- Detects library preloading mechanisms that could be used for privilege escalation or defense evasion


### Notes
!!! note
    - Monitors `/etc/ld.so.preload` for system-wide library preloading
    - Scans environment configuration files for LD_PRELOAD variables
    - Checks systemd service files for LD_PRELOAD in Environment directives
    - Requires administrator access to read system configuration files


### Configuration
```ini
[linux_ld_preload]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_ld_preload": [
    {
        "Source": "/etc/ld.so.preload",
        "Configuration": "/opt/custom/libhook.so"
    },
    {
        "Source": "/etc/environment",
        "Configuration": "LD_PRELOAD=/tmp/malicious.so"
    },
    {
        "Source": "/etc/systemd/system/custom.service",
        "Configuration": "Environment=LD_PRELOAD=/lib/backdoor.so"
    }
]
```

| Key | Description |
| --- | ----------- |
| Source | File path where the LD_PRELOAD configuration was found |
| Configuration | Content of the LD_PRELOAD configuration line |


### Security Implications
Adversaries may use LD_PRELOAD to hijack shared library loading and execute malicious code with elevated privileges. This technique can be used for:

- **Privilege Escalation**: Preloading malicious libraries before legitimate system libraries
- **Defense Evasion**: Hooking system calls to hide malicious activity
- **Persistence**: Ensuring malicious code runs with every process execution
- **Credential Theft**: Intercepting authentication functions
- **Rootkit Functionality**: Hiding files, processes, and network connections


### Detection Strategy
Monitor for:

- Unexpected entries in `/etc/ld.so.preload`
- LD_PRELOAD variables in `/etc/environment` or `/etc/profile.d/*`
- Suspicious library paths (e.g., `/tmp`, `/dev/shm`, world-writable directories)
- LD_PRELOAD in systemd service unit files
- Libraries loaded from non-standard locations


### MITRE ATT&CK Mapping
- [T1574.006 Hijack Execution Flow: Dynamic Linker Hijacking](https://attack.mitre.org/techniques/T1574/006/)


### References
- [ld.so man page](https://man7.org/linux/man-pages/man8/ld.so.8.html)
- [LD_PRELOAD Rootkit Detection](https://www.giac.org/paper/gcux/148/detection-ld-preload-based-user-land-rootkits/103290)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))
