# linux_systemd_timers

### Description
- Active systemd timer units
- Monitors scheduled tasks that could be used for persistence or privilege escalation


### Notes
!!! note
    - Lists all active systemd timers regardless of user
    - Timers are the modern systemd replacement for traditional cron jobs
    - Requires systemd to be running (standard on most modern Linux distributions)


### Configuration
```ini
[linux_systemd_timers]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_systemd_timers": [
    "apt-daily-upgrade.timer",
    "apt-daily.timer",
    "e2scrub_all.timer",
    "fstrim.timer",
    "logrotate.timer",
    "man-db.timer",
    "systemd-tmpfiles-clean.timer",
    "ua-timer.timer"
]
```


### Security Implications
Adversaries may use systemd timers to maintain persistence or execute malicious code at specific intervals. Monitoring systemd timers can help detect:

- Unauthorized scheduled tasks
- Malicious scripts scheduled for execution
- Persistence mechanisms disguised as system maintenance tasks
- Privilege escalation attempts via timer-based execution
- Unusual or suspicious timer configurations


### Detection Strategy
Monitor for:

- New timer units that weren't present in previous snapshots
- Timers executing scripts from unusual locations:
  - `/tmp`, `/dev/shm`, `/var/tmp`
  - User home directories
  - Web server directories
- Timers with very frequent execution (every minute/second)
- User-created timers in non-standard locations
- Timers running as root with suspicious commands
- Timer units without corresponding service files
- Timers executing network commands or downloads

### Baseline Recommendations
- Document standard system timers (apt-daily, fstrim, logrotate, etc.)
- Review timer unit files in `/etc/systemd/system` and `/lib/systemd/system`
- Use `systemctl list-timers --all` to see disabled timers too
- Check associated service files for each timer


### MITRE ATT&CK Mapping
- [T1053.006 Scheduled Task/Job: Systemd Timers](https://attack.mitre.org/techniques/T1053/006/)


### References
- [Systemd Timer Documentation](https://www.freedesktop.org/software/systemd/man/systemd.timer.html)
- [Arch Linux systemd/Timers Guide](https://wiki.archlinux.org/title/Systemd/Timers)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))
