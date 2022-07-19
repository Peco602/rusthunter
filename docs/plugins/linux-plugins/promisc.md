# linux_promisc

### Description
This plugin provides a list of network interfaces in promiscous mode on a Linux machine. Adversaries may sniff network traffic to capture information about an environment, including authentication material passed over the network. Network sniffing refers to using the network interface on a system to monitor or capture information sent over a wired or wireless connection. An adversary may place a network interface into promiscuous mode to passively access data in transit over the network, or use span ports to capture a larger amount of data.

### Parameters
| Name | Options | Default | Description |
| ---- | ------- | ------- | ----------- |
| enabled | true/false | true | Plugin status |

### Returned values
Array of interfaces in promiscous mode:

- *interface 1*
- *interface 2*

### MITRE ATT&CK Mapping
- [T1040 Network Sniffing](https://attack.mitre.org/techniques/T1040/)

### Authors
- Andrea Vozza ([landerover](https://github.com/landerover))