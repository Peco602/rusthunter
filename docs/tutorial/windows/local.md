# Windows :: Local snapshot

RustHunter can take a snapshot of the local machine based on a specific `config.ini` configuration file. You can optionally select a tag (via `-SnapshotTag <TAG>`) to be added to the snapshot to distinguish them:

```console
PS C:\Users\user\rusthunter-main> .\rusthunter.ps1 local -ConfigFile .\config.ini -SnapshotTag PRE-PATCHING

  /#######                        /##     /##   /##                       /##                          
 | ##__  ##                      | ##    | ##  | ##                      | ##                          
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        

 [+] Collecting data 
[*] Executing windows_users plugin
[+] Done
[*] Executing windows_administrators plugin
[+] Done
[*] Executing windows_tcp_listen plugin
[+] Done
[*] Executing windows_udp_listen plugin
[+] Done
[*] Executing windows_autoruns plugin
[+] Done
[+] Snapshot file created: PRE-PATCHING_192.168.100.3_20220722-134005.json
```