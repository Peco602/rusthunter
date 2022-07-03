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
user@master-node:~/rusthunter$ sudo ./rusthunter.sh install

  /#######                        /##     /##   /##                       /##
 | ##__  ##                      | ##    | ##  | ##                      | ##
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/

 [+] Installing executable
 [+] Building launcher docker image

 ...

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

...

[macos_users]
# OS: macOS
# Data: Users
enabled   = true
```


## 4. Hosts File

Edit the `hosts` file to select the target nodes to be included in the snapshot. 

1. Add a line for each Linux machine. The selected user must be able to access the machine via SSH and must be included in the `sudoers`:

    ```ini
    192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

    [linux]

    [windows]

    [macos]

    ```

2. Add a line for each Windows machine. The selected user must be able to access the machine via WinRM and should be included in the `Administrators` group:

    ```ini
    192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
    192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
    192.168.1.103 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=kerberos ansible_user=windows_user ansible_password=P4ssw0rd123@

    [linux]

    [windows]

    [macos]
    
    ```

3. Add a line for each MacOS machine. The selected user must be able to access the machine via SSH and must be included in the `sudoers`:

    ```ini
    192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
    192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
    192.168.1.103 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

    [linux]

    [windows]

    [macos]
    
    ```

3. Add each machine IP or hostname to the corresponding group, i.e., `[linux]`, `[windows]` and `[macos]`:

    ```ini
    192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
    192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
    192.168.1.103 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

    [linux]
    192.168.1.101

    [windows]
    192.168.1.102

    [linux]
    192.168.1.103
    ```


## 5. Snapshotting

Take the snapshot based on the custom `host` and `config` files:

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh global -h ./hosts -c ./config

  /#######                        /##     /##   /##                       /##
 | ##__  ##                      | ##    | ##  | ##                      | ##
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/

 [+] Creating snapshots directory

 [+] Collecting data

PLAY [Run RustHunter on Linux machines] ****************************************

...

PLAY [Run RustHunter on macOS machines] ****************************************

...

PLAY [Run RustHunter on Windows machines] **************************************

...

 [+] Merging data 
[*] Reading file: "launcher\snapshots\snapshot-192.168.1.101.json"
[*] Reading file: "launcher\snapshots\snapshot-192.168.1.102.json"
[*] Reading file: "launcher\snapshots\snapshot-192.168.1.103.json"
[+] Merged snapshots file: snapshot-20220420_121525.json

```


## 6. Comparison

Compare the two environmental snapshots by checking the statistics (use the `-s` parameter):

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh compare -s -i ./snapshot-20220410_131824.json -c ./snapshot-20220420_121525.json

  /#######                        /##     /##   /##                       /##
 | ##__  ##                      | ##    | ##  | ##                      | ##
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/ 

Host               Plugin                    Initial    Current
====================================================================================================
192.168.1.101      linux_users               2          2          [+] Ok

...

192.168.1.101      linux_tcp_listen          5          5          [+] Ok
192.168.1.101      linux_udp_listen          2          2          [+] Ok
----------------------------------------------------------------------------------------------------
192.168.1.102      windows_administrators    2          2          [+] Ok

...

192.168.1.102      windows_tcp_listen        26         27         [!] Mismatch
192.168.1.102      windows_udp_listen        37         37         [+] Ok
192.168.1.102      windows_users             6          6          [+] Ok
----------------------------------------------------------------------------------------------------
192.168.1.103      macos_users               5          5          [+] Ok

...

----------------------------------------------------------------------------------------------------
```

It seems there is an additional TCP listening port on the Windows host `192.168.1.102`. Get the details by filtering by host and plugin (use the `-h` and `-p` parameters):

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh compare -h 192.168.1.102 -p windows_tcp_listen -i ./snapshot-20220410_131824.json -c ./snapshot-20220420_121525.json

  /#######                        /##     /##   /##                       /##
 | ##__  ##                      | ##    | ##  | ##                      | ##
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/ 

--- original
+++ modified
@@ -22,7 +22,7 @@
   {
+    "LocalAddress": "::",
+    "LocalPort": 5022,
+    "ProcessName": "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge_updater.exe"
   },
   {
     "LocalAddress": "::",
```

An unexpected process `msedge_updater.exe` started to listen on the TCP port `5022`.
