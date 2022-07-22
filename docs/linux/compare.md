# Linux :: Compare

RustHunter allows to compare both local and global snapshots to identify any deviation that could be related to a threat.

## 1. Statistics

Check the comparison statistics (use the `-ShowStatistics` parameter) to get a brief overview of the differences between the two snapshots under analysis:

```console
PS C:\Users\user\rusthunter-main> .\rusthunter.ps1 compare -ShowStatistics -InitialSnapshot .\PRE-PATCHING_20220410-131824.json -CurrentSnapshot .\POST-PATCHING_20220420-121525.json

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

It seems there is an additional TCP listening port on the Windows host `192.168.1.102`.


## 2. Differences

Get the difference details via filtering by host and plugin (use the `-h` and `-p` parameters):

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh compare -h 192.168.1.102 -p windows_tcp_listen -i ./PRE-PATCHING_20220410-131824.json -c ./POST-PATCHING_20220420-121525.json

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
