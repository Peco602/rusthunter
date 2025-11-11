# linux_kernel_modules

### Description
- Loaded kernel modules
- Monitors kernel modules that could be used for rootkits or malicious drivers


### Notes
!!! note
    - Lists all currently loaded kernel modules using `lsmod`
    - Kernel modules run with full system privileges
    - Malicious kernel modules can hide processes, files, and network connections


### Configuration
```ini
[linux_kernel_modules]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_kernel_modules": [
    {
        "Module": "nf_conntrack",
        "Size": "139264",
        "UsedBy": "xt_conntrack"
    },
    {
        "Module": "xt_MASQUERADE",
        "Size": "20480",
        "UsedBy": "1"
    },
    {
        "Module": "iptable_nat",
        "Size": "16384",
        "UsedBy": "1"
    },
    {
        "Module": "nf_nat",
        "Size": "45056",
        "UsedBy": "xt_MASQUERADE,iptable_nat"
    }
]
```

| Key | Description |
| --- | ----------- |
| Module | Name of the kernel module |
| Size | Size of the module in bytes |
| UsedBy | Number of references or list of modules using this module |


### Security Implications
Kernel modules operate at the highest privilege level and can completely compromise system security. Adversaries may load malicious kernel modules to:

- **Rootkit Installation**: Hide malicious processes, files, and network connections
- **Defense Evasion**: Bypass security controls and evade detection
- **Privilege Escalation**: Gain root-level access to the system
- **Persistence**: Maintain access across reboots via module autoloading
- **System Information Discovery**: Enumerate system configuration and security tools

### Detection Strategy
Monitor for:

- Unknown or suspicious kernel module names
- Modules loaded from non-standard locations (not in `/lib/modules`)
- New modules appearing that weren't in previous snapshots
- Modules with suspicious names (random characters, similar to legitimate names)
- Known malicious module names (rootkit signatures)
- Unsigned modules on systems with module signing enabled

### Notes
!!! warning
    - Kernel rootkits are extremely difficult to detect from userspace
    - Compare against known good baselines
    - Unsigned modules may indicate compromise on systems with Secure Boot
    - Some legitimate drivers may have cryptic names


### MITRE ATT&CK Mapping
- [T1082 System Information Discovery](https://attack.mitre.org/techniques/T1082/)
- [T1014 Rootkit](https://attack.mitre.org/techniques/T1014/)


### References
- [Linux Kernel Module Programming Guide](https://sysprog21.github.io/lkmpg/)
- [Detecting Linux Kernel Rootkits](https://www.sans.org/white-papers/33649/)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))
