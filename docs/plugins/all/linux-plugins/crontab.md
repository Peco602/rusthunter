# linux_crontab

### Description
- List of active crontabs


### Notes
!!! note
    - Requires administrator access to get cron data from all users.
    - Refer to [Crontab.guru](https://crontab.guru/) to easily decode cron expressions.

    
### Requirements
The following files are required:

| File | Description |
| ---- | ----------- |
| *crontab.sh* | Bash [script](https://stackoverflow.com/questions/134906/how-do-i-list-all-cron-jobs-for-all-users) collecting crontab lines for all users |

in the path "*launcher/ansible/roles/linux/files*".


### Configuration
```ini
[linux_crontab]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_crontab": [
    "10  3     *  *  *  root   test -e /run/systemd/system || SERVICE_MODE=1 /sbin/e2scrub_all -A -r",
    "25  6     *  *  *  root   /etc/cron.daily/0anacron",
    "25  6     *  *  *  root   /etc/cron.daily/apache2",
    "25  6     *  *  *  root   /etc/cron.daily/apport",
    "25  6     *  *  *  root   /etc/cron.daily/apt-compat",
    "25  6     *  *  *  root   /etc/cron.daily/cracklib-runtime",
    "25  6     *  *  *  root   /etc/cron.daily/dpkg",
    "25  6     *  *  *  root   /etc/cron.daily/logrotate",
    "25  6     *  *  *  root   /etc/cron.daily/man-db",
    "30  3     *  *  0  root   test -e /run/systemd/system || SERVICE_MODE=1 /usr/lib/x86_64-linux-gnu/e2fsprogs/e2scrub_all_cron",
    "30  7-23  *  *  *  root   [ -x /etc/init.d/anacron ] && if [ ! -d /run/systemd/system ]; then /usr/sbin/invoke-rc.d anacron start >/dev/null; fi",
    "47  6     *  *  7  root   /etc/cron.weekly/0anacron",
    "47  6     *  *  7  root   /etc/cron.weekly/man-db",
    "52  6     1  *  *  root   /etc/cron.monthly/0anacron",
]
```


### MITRE ATT&CK Mapping
- [T1053.003 Scheduled Task/Job: Cron](https://attack.mitre.org/techniques/T1053/003/)


### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))