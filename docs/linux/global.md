
# Linux :: Global snapshot

RustHunter can easily take a snapshot of a large environment. The only requirement is the reachibility of all the target nodes from the master node via SSH or WinRM. The `hosts` file must be properly modified to select the target nodes to be included in the global snapshot. 

!!! note
    It is suggested to prepare in advance multiple hosts files and then to select each time the most suitable for the snapshot.
    
## 1. Linux target nodes

Add a line for each Linux machine. The selected user must be able to access the machine via SSH and must be included in the `sudoers`:

```
192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

[linux]
192.168.1.101

[windows]

[macos]

```

## 2. Windows target nodes

Add a line for each Windows machine. The selected user must be able to access the machine via WinRM and should be included in the `Administrators` group:

```
192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@

[linux]
192.168.1.101

[windows]
192.168.1.102

[macos]

```

## 3. macOS target nodes

Add a line for each MacOS machine. The selected user must be able to access the machine via SSH and must be included in the `sudoers`:

```
192.168.1.101 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@
192.168.1.102 ansible_connection=winrm ansible_port=5985 ansible_winrm_transport=ntlm ansible_user=windows_user ansible_password=P4ssw0rd123@
192.168.1.103 ansible_connection=ssh ansible_user=linux_user ansible_ssh_password=P4ssw0rd123@ ansible_become_pass=P4ssw0rd123@

[linux]
192.168.1.101

[windows]
192.168.1.102

[macos]
192.168.1.103 
```

## 4. Snapshotting

Take the snapshot based on the custom `host` and `config.ini` files. You can optionally select a tag (via `-t <TAG>`) to be added to the snapshot to distinguish them:

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh global -h ./hosts -c ./config.ini -t PRE-PATCHING

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
[*] Reading file: "launcher\snapshots\PRE-PATCHING_192.168.1.101_20220420-121524.json"
[*] Reading file: "launcher\snapshots\PRE-PATCHING_192.168.1.102_20220420-121524.json"
[*] Reading file: "launcher\snapshots\PRE-PATCHING_192.168.1.103_20220420-121524.json"
[+] Merged snapshots file: PRE-PATCHING_20220420-121525.json
```