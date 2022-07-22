# Windows :: Setup

The following steps allow the setup of RustHunter on a Windows master node.

## System requirements

- Windows >= 10
- 8+ GB RAM
- 2+ CPUs
- [`Docker Desktop`](https://docs.docker.com/desktop/windows/install/)


## 1. Download

Download and extract the RustHunter package directly from the official GitHub repository [`releases`](https://github.com/Peco602/rusthunter/releases).

```ps1con
PS C:\Users\user> wget https://github.com/Peco602/rusthunter/archive/main.zip -UseBasicParsing -OutFile rusthunter.zip

StatusCode        : 200
StatusDescription : OK
Content           : {80, 75, 3, 4...}
RawContent        : HTTP/1.1 200 OK
                    Access-Control-Allow-Origin: https://render.githubusercontent.com
                    content-disposition: attachment; filename=rusthunter-main.zip
                    Content-Security-Policy: default-src 'none'; style-sr...
Headers           : {[Access-Control-Allow-Origin, https://render.githubusercontent.com], [content-disposition, attachment; filename=rusthunter-main.zip], [Content-Security-Policy, default-src 'none'; style-src 'unsafe-inline'; sandbox], [Strict-Transport-Security,
                    max-age=31536000]...}
RawContentLength  : 4338172

PS C:\Users\user>  Expand-Archive .\rusthunter.zip -DestinationPath .
```

RustHunter can be completely managed on a Windows master node via the `rusthunter.ps1` PowerShell script. 

 
## 2. Building [Optional]

The Rusthunter repository already provides the required compiled binaries. If you prefer to check the Rust code by yourself and recompile it, you can easily do it:

```ps1con
PS C:\Users\user> powershell -ep bypass
Windows PowerShell
Copyright (C) Microsoft Corporation. All rights reserved.

PS C:\Users\user> cd rusthunter-main
PS C:\Users\user\rusthunter-main> .\rusthunter.ps1 build

  /#######                        /##     /##   /##                       /##
 | ##__  ##                      | ##    | ##  | ##                      | ##
 | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######
 | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##
 | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/
 | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##
 | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##
 |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/

 [+] Building release for Linux target

// ... skipped ...

 [+] Building release for macOS target

// ... skipped ...

 [+] Building release for Windows target

// ... skipped ...

 [+] Moving executables to the launcher folders
 [+] Installing executable
 [+] Cleaning up
```


## 3. Installation

Execute the installation command:

!!! note
    Open the Windows PowerShell session as Administrator.

```ps1con
PS C:\Users\user> powershell -ep bypass
Windows PowerShell
Copyright (C) Microsoft Corporation. All rights reserved.

PS C:\Users\user> cd rusthunter-main
PS C:\Users\user\rusthunter-main> .\rusthunter.ps1 install

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


## 4. Check

If the setup has been successfully completed you should find the `rusthunter.exe` binary available in your PATH:

```console
PS C:\Users\user\rusthunter-main> rusthunter.exe
RustHunter 1.0
Giovanni Pecoraro <giovanni1.pecoraro@protonmail.com>
Environmental baseline comparison tool

USAGE:
    rusthunter [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    Enable verbose output
    -V, --version    Print version information

SUBCOMMANDS:
    compare    Compare two snapshot files
    help       Print this message or the help of the given subcommand(s)
    list       List all the available plugins
    merge      Merge snapshot files in a directory
    run        Take a system snapshot
```