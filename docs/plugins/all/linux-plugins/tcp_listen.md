# linux_tcp_listen

### Description
- Listening TCP ports


### Notes
!!! note
    Requires administrator access to get the process names.


### Configuration
```ini
[linux_tcp_listen]
enabled   = false
```


### Returned values
```json
"linux_tcp_listen": [
    {
        "Port": "127.0.0.1:631",
        "Process": "cupsd",
        "User": "root"
    },
    {
        "Port": "[::1]:631",
        "Process": "cupsd",
        "User": "root"
    },
    {
        "Port": "*:22",
        "Process": "sshd",
        "User": "root"
    },
    {
        "Port": "*:22",
        "Process": "sshd",
        "User": "root"
    },
    {
        "Port": "127.0.0.53:53",
        "Process": "systemd-r",
        "User": "systemd-resolve"
    }
]
```

| Key | Description |
| --- | ----------- |
| Process | Name of the process |
| Port | Listening port on a specific interface  |
| User | User running the process |


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))