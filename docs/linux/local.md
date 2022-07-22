# Linux :: Local snapshot

RustHunter can take a snapshot of the local machine based on a specific `config.ini` configuration file. You can optionally select a tag (via `-t <TAG>`) to be added to the snapshot to distinguish them:

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh local -c ./config.ini -SnapshotTag PRE-PATCHING

  /#######                        /##     /##   /##                       /##                          
 | ##__  ##                      | ##    | ##  | ##                      | ##                          
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        

 [+] Collecting data 
[*] Executing linux_users plugin
[+] Done
[*] Executing linux_root plugin
[+] Done
[*] Executing linux_tcp_listen plugin
[+] Done
[*] Executing linux_suid plugin
[+] Done
[*] Executing linux_guid plugin
[+] Done
[*] Executing linux_crontab plugin
[+] Done
[+] Snapshot file created: PRE-PATCHING_192.168.100.2_20220722-134005.json
```