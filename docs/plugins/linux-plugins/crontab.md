# linux_crontab

### Description
This plugin provides the list of the active scheduled tasks on a Linux machine. 

Adversaries may abuse the cron utility to perform task scheduling for initial or recurring execution of malicious code. The cron utility is a time-based job scheduler for Unix-like operating systems. The crontab file contains the schedule of cron entries to be run and the specified times for execution. Any crontab files are stored in operating system-specific file paths.

### Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |

### Files
Path: *launcher/ansible/roles/linux/files*

| File | Description |
| ---- | ----------- |
| *crontab.sh* | Bash [script](https://stackoverflow.com/questions/134906/how-do-i-list-all-cron-jobs-for-all-users) collecting crontab lines for all users |

### Returned values
Array of cron expressions:

- *15  14  1  *  *   root   task1*
-  *0  22  *  *  *   root   task2*
-  *1   *  *  *  *   user   task3*

### MITRE ATT&CK Mapping
- [T1053.003 Scheduled Task/Job: Cron](https://attack.mitre.org/techniques/T1053/003/)

### Notes
!!! note
    - Requires administrator access to get cron data from all users.
    - Refer to [Crontab.guru](https://crontab.guru/) to easily decode cron expressions.

### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))