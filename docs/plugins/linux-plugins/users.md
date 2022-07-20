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


### MITRE ATT&CK Mapping
- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))