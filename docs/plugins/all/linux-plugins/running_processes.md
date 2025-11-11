# linux_running_processes

### Description
- Running processes with full command line arguments
- Monitors active processes for malicious executables and suspicious activity


### Notes
!!! note
    - Uses `ps aux` to capture detailed process information
    - Includes full command line with arguments
    - Shows CPU and memory usage per process
    - Captures process state and runtime information
    - Provides user context for each process


### Configuration
```ini
[linux_running_processes]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_running_processes": [
    {
        "User": "root",
        "PID": "1",
        "CPU": "0.0",
        "MEM": "0.1",
        "VSZ": "169000",
        "RSS": "13204",
        "TTY": "?",
        "STAT": "Ss",
        "START": "Nov10",
        "TIME": "0:03",
        "Command": "/sbin/init splash"
    },
    {
        "User": "www-data",
        "PID": "2847",
        "CPU": "0.5",
        "MEM": "2.3",
        "VSZ": "1234567",
        "RSS": "234567",
        "TTY": "?",
        "STAT": "S",
        "START": "14:23",
        "TIME": "1:45",
        "Command": "/usr/sbin/apache2 -k start"
    }
]
```

| Key | Description |
| --- | ----------- |
| User | User running the process |
| PID | Process ID |
| CPU | CPU usage percentage |
| MEM | Memory usage percentage |
| VSZ | Virtual memory size in KB |
| RSS | Resident set size (physical memory) in KB |
| TTY | Controlling terminal |
| STAT | Process state (S=sleeping, R=running, Z=zombie, etc.) |
| START | Process start time |
| TIME | Cumulative CPU time |
| Command | Full command line with arguments |


### Security Implications
Process monitoring is fundamental for detecting malicious activity. Adversaries may run:

- **Malware Execution**: Trojans, ransomware, cryptominers
- **Remote Access Tools**: Backdoors, reverse shells, RATs
- **Credential Dumping**: Memory scraping tools (mimikatz, etc.)
- **Reconnaissance Tools**: Network scanners, enumeration scripts
- **Privilege Escalation**: Exploit tools, kernel exploits
- **Lateral Movement**: Remote execution tools (psexec, wmi, etc.)
- **Defense Evasion**: Process injection, hollowing, masquerading

### Detection Strategy
Monitor for:

- Processes running from unusual locations (`/tmp`, `/dev/shm`, user home directories)
- Suspicious process names (random characters, typosquatting system processes)
- Processes with unusual parent-child relationships
- System processes running with incorrect user context
- High CPU/memory usage from unknown processes
- Processes with obfuscated or encoded command lines
- Known malicious process names or hashes
- Scripts running from web server directories

### Red Flags
⚠️ **High Priority Indicators:**

- Shells spawned by web servers or services
- Processes named similar to system processes (scvhost, systend)
- Executables in `/tmp` or `/dev/shm`
- Base64 encoded commands in process arguments
- Curl/wget downloading and executing files
- Python/perl/bash one-liners with network connections
- Memory dumping tools (procdump, gcore, etc.)

### Baseline Recommendations
Establish baselines for:

- Normal running processes and their command lines
- Expected processes per user type
- Typical resource usage patterns
- Standard service configurations
- Authorized administrative tools


### MITRE ATT&CK Mapping
- [T1057 Process Discovery](https://attack.mitre.org/techniques/T1057/)
- [T1106 Native API](https://attack.mitre.org/techniques/T1106/)


### References
- [ps command documentation](https://man7.org/linux/man-pages/man1/ps.1.html)
- [Linux Process States](https://idea.popcount.org/2012-12-11-linux-process-states/)
- [Detecting Malicious Processes](https://www.sans.org/blog/detecting-malicious-software-through-process-analysis/)


### Authors
- theRedCount ([theRedCount](https://github.com/theRedCount))
