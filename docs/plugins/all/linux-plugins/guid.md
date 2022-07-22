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

### MITRE ATT&CK Mapping
- [T1548.001 Abuse Elevation Control Mechanism: Setuid and Setgid](https://attack.mitre.org/techniques/T1548/001/)

### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))