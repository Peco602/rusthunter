# Quickstart :: Linux

The following steps allow the deployment of RustHunter on a Linux master node.


## System requirements

- Linux >= 18.4
- 8+ GB RAM
- 2+ CPUs


## 1. Cloning

Clone the RustHunter repository directly from [`GitHub`](https://github.com/Peco602/rusthunter/):

```console
user@master-node:~$ git clone https://github.com/Peco602/rusthunter

```


## 2. Installation

Install the RustHunter executable on your system. RustHunter can be completely managed on a Linux master node via the `rusthunter.sh` Bash script. 

```console
user@master-node:~$ cd rusthunter
user@master-node:~$ sudo ./rusthunter.sh install

 ______          _   _   _             _            
 | ___ \        | | | | | |           | |           
 | |_/ /   _ ___| |_| |_| |_   _ _ __ | |_ ___ _ __ 
 |    / | | / __| __|  _  | | | | '_ \| __/ _ \ '__|
 | |\ \ |_| \__ \ |_| | | | |_| | | | | ||  __/ |   
 \_| \_\__,_|___/\__\_| |_/\__,_|_| |_|\__\___|_|  

 [+] Installing executable
 [+] Successfully installed

```


## 3. Configuration File

Edit the `config` file to enable/configure specific data collection to be included in the environmental snapshots.

```ini
[linux_users]
# OS: Linux
# Data: Users
enabled = true

[linux_tcp_listen]
# OS: Linux
# Data: TCP listening ports
enabled = true

[linux_udp_listen]
# OS: Linux
# Data: UDO listening ports
enabled = true

...

[windows_autoruns]
# OS: Windows
# Data: Autorun entries
enabled                   = true
boot_execute              = true
appinit_dlls              = false
explorer_addons           = false
sidebar_gadgets           = false
image_hijacks             = false
internet_explorer_addons  = false
known_dlls                = false
logon_startups            = false
wmi_entries               = false
winsock_protocol          = false
codecs                    = false
printer_monitor_dlls      = false
lsa_security_providers    = false
autostart_services        = false
scheduled_tasks           = false
winlogon_entries          = false

[windows_tcp_listen]
# OS: Windows
# Data: TCP listening ports
enabled   = true

[windows_udp_listen]
# OS: Windows
# Data: UDP listening ports
enabled   = true
```


## 4. Hosts File

Edit the `hosts` file to select the target nodes to be included in the snapshot. 

1. Add a line for each Linux machine. The selected user must be able to access the machine via SSH and must be included in the `sudoers`:

    ```
    192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

    [linux]

    [windows]

    ```

2. Add a line for each Windows machine. The selected user must be able to access the machine via WinRM and should be included in the `Administrators` group:

    ```
    192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
    192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
    192.168.1.103 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=kerberos ansible_user=windows_user ansible_password=P4ssw0rd123@

    [linux]

    [windows]

    ```

3. Add each machine IP or hostname to the corresponding group, i.e., `Linux` or `Windows`:

    ```
    192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
    192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
    192.168.1.103 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=kerberos ansible_user=windows_user ansible_password=P4ssw0rd123@

    [linux]
    192.168.1.101

    [windows]
    192.168.1.102
    192.168.1.103
    ```


## 5. Snapshotting

Take the snapshot based on the custom `host` and `config` files:

```console
user@master-node:~$ sudo ./rusthunter.sh global -h ./hosts -c ./config

 ______          _   _   _             _            
 | ___ \        | | | | | |           | |           
 | |_/ /   _ ___| |_| |_| |_   _ _ __ | |_ ___ _ __ 
 |    / | | / __| __|  _  | | | | '_ \| __/ _ \ '__|
 | |\ \ |_| \__ \ |_| | | | |_| | | | | ||  __/ |   
 \_| \_\__,_|___/\__\_| |_/\__,_|_| |_|\__\___|_|  

 [+] Creating snapshots directory

 [+] Collecting data

...

 [*] Merging data 
Reading file: "launcher/snapshots/snapshot-192.168.1.101.json"
Reading file: "launcher/snapshots/snapshot-192.168.1.102.json"
Reading file: "launcher/snapshots/snapshot-192.168.1.103.json"
Merged snapshots file: snapshot-20220420_121525.json

```


## 6. Comparison

Compare two environmental snapshots to find differences:

```console
user@master-node:~$ sudo ./rusthunter.sh compare -i ./snapshot-20220410_131824.json -c ./snapshot-20220420_121525.json

 ______          _   _   _             _            
 | ___ \        | | | | | |           | |           
 | |_/ /   _ ___| |_| |_| |_   _ _ __ | |_ ___ _ __ 
 |    / | | / __| __|  _  | | | | '_ \| __/ _ \ '__|
 | |\ \ |_| \__ \ |_| | | | |_| | | | | ||  __/ |   
 \_| \_\__,_|___/\__\_| |_/\__,_|_| |_|\__\___|_|  


No differences

```


