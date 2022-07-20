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

### MITRE ATT&CK Mapping
- [T1548.001 Abuse Elevation Control Mechanism: Setuid and Setgid](https://attack.mitre.org/techniques/T1548/001/)

### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))