# Windows :: Update

RustHunter can be easily updated via the `update` subcommand.

!!! note
    It requires `git` to be installed.

```console
PS C:\Users\user\rusthunter-main> .\rusthunter.ps1 update

  /#######                        /##     /##   /##                       /##
 | ##__  ##                      | ##    | ##  | ##                      | ##
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/

 [+] Downloading latest updates

// ... skipped ...

 [+] Installing new executable
 [+] Successfully updated
```

In case `git` is not available, the update can be performed via a new [setup](setup.md).