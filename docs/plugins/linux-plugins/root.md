# linux_root

### Description
- Local root accounts


### Configuration
```ini
[linux_root]
enabled = true
```

| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |


### Returned values
```json
"linux_root": [
    "root"
]
```


### MITRE ATT&CK Mapping
- [T1136.001 Create Account: Local Account](https://attack.mitre.org/techniques/T1136/001/)


### Authors
- Giovanni Pecoraro ([Peco602](https://github.com/peco602))