# Linux :: Setup

The following steps allow the setup of RustHunter on a Linux master node.


## System requirements

- Linux >= 18.4
- 8+ GB RAM
- 2+ CPUs


## 1. Cloning

Clone the RustHunter repository directly from [`GitHub`](https://github.com/Peco602/rusthunter/):

```console
user@master-node:~$ git clone https://github.com/Peco602/rusthunter

```

RustHunter can be then completely managed on a Linux master node via the `rusthunter.sh` Bash script. 


## 2. Building [Optional]

The Rusthunter repository already provides the required compiled binaries. If you prefer to check the Rust code by yourself and recompile it, you can easily do it:

```console
user@master-node:~$ cd rusthunter
user@master-node:~/rusthunter$ sudo ./rusthunter.sh build

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


## 4. Check

If the setup has been successfully completed you should find the `rusthunter` binary available in your PATH:

```console
user@master-node:~/rusthunter$ rusthunter
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