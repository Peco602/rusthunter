# linux_promisc

### Description
- Network interface in promiscuous mode


### Configuration
```ini
[linux_promisc]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_promisc": [
    "eth0"
]
```

### MITRE ATT&CK Mapping
- [T1040 Network Sniffing](https://attack.mitre.org/techniques/T1040/)


### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))