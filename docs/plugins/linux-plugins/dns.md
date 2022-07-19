# linux_dns

### Description
This plugin provides the list of DNS servers in use on a Linux machine. 

Adversaries may replace the local DNS server with a compromised one.

### Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |

### Returned values
Array of hostnames or IPs addresses:

- *dns server 1*
- *dns server 2*

### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))