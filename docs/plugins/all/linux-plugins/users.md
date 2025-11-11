# linux_users

### Description
- Local user accounts


### Configuration
```ini
[linux_users]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_users": [
    "_apt",
    "avahi",
    "avahi-autoipd",
    "backup",
    "bin",
    "colord",
    "cups-pk-helper",
    "daemon",
    "dnsmasq",
    "games",
    "gdm",
    "geoclue",
    "gnats",
    "gnome-initial-setup",
    "hplip",
    "irc",
    "kernoops",
    "list",
    "lp",
    "mail",
    "man",
    "messagebus",
    "news",
    "nm-openvpn",
    "nobody",
    "proxy",
    "pulse",
    "root",
    "rtkit",
    "saned",
    "speech-dispatcher",
    "sshd",
    "sssd",
    "sync",
    "sys",
    "syslog",
    "systemd-coredump",
    "systemd-network",
    "systemd-resolve",
    "systemd-timesync",
    "tcpdump",
    "tss",
    "usbmux",
    "uucp",
    "uuidd",
    "whoopsie",
    "www-data"
]
```


### Security Implications
Local user accounts are a common target for adversaries seeking to establish persistence, escalate privileges, or move laterally within a network. Monitoring user accounts can help detect:

- **Unauthorized Account Creation**: New user accounts created by adversaries for persistence
- **Backdoor Accounts**: Hidden or disguised user accounts with elevated privileges
- **Account Modification**: Changes to existing accounts (shells, home directories, UIDs)
- **Suspicious System Accounts**: Service accounts with interactive shells or unusual configurations
- **Dormant Account Activation**: Previously inactive accounts that suddenly become active

### Detection Strategy
Compare snapshots to identify:

- New user accounts that weren't previously present
- Changes to user account properties (shell, home directory, UID/GID)
- Service accounts with login shells (should typically have `/usr/sbin/nologin` or `/bin/false`)
- Accounts with UID 0 (root privileges) other than the root account


### MITRE ATT&CK Mapping
- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)
- [T1098 Account Manipulation](https://attack.mitre.org/techniques/T1098/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))