#Requires -RunAsAdministrator

Param(
    [Parameter(Position = 0)]
    [string]
    $Subcommand = $null,

    [string]
    $HostsFile = $null,

    [string]
    $ConfigFile = $null,
    
    [string]
    $InitialSnapshot = $null,

    [string]
    $CurrentSnapshot = $null,

    [switch]
    $UnitTests = $null,

    [switch]
    $IntegrationTests = $null,

    [switch]
    $ValidationTests = $null
)

###########################################################
# VARIABLES
${EXECUTABLE_NAME}="rusthunter.exe"

${BUILDER_IMAGE_NAME}="rusthunter/builder"
${BUILDER_IMAGE_PATH}=".\builder"

${LAUNCHER_IMAGE_NAME}="rusthunter/launcher"
${LAUNCHER_IMAGE_PATH}=".\launcher"

${INSTALLATION_PATH}="c:\windows\system32"
${APP_PATH}=".\app"
${ANSIBLE_PATH}=".\launcher\ansible"
${LINUX_BINARIES_PATH}="${ANSIBLE_PATH}\roles\linux\files"
${WINDOWS_BINARIES_PATH}="${ANSIBLE_PATH}\roles\windows\files"
${SNAPSHOT_PATH}=".\launcher\snapshots"

${DEFAULT_CONFIG_FILE}="config"
${DEFAULT_HOSTS_FILE}="hosts"
###########################################################

function ShowBanner {
    cls
    Write-Host " ______          _   _   _             _              " -ForegroundColor blue
    Write-Host " | ___ \        | | | | | |           | |             " -ForegroundColor blue
    Write-Host " | |_/ /   _ ___| |_| |_| |_   _ _ __ | |_ ___ _ __   " -ForegroundColor blue
    Write-Host " |    / | | / __| __|  _  | | | | '_ \| __/ _ \ '__|  " -ForegroundColor blue
    Write-Host " | |\ \ |_| \__ \ |_| | | | |_| | | | | ||  __/ |     " -ForegroundColor blue
    Write-Host " \_| \_\__,_|___/\__\_| |_/\__,_|_| |_|\__\___|_|     " -ForegroundColor blue
    Write-Host ""
}

function ShowHelp {
    Write-Host "usage: $0 [SUBCOMMAND] [ARGS]"
    Write-Host
    Write-Host "SUBCOMMANDS:"
    Write-Host
    Write-Host "     install           Install RustHunter on the system"
    Write-Host "     list              List all available plugins"
    Write-Host "     local             Take a local snapshot"
    Write-Host "     global            Take a global snapshot based on hosts file (requires Docker)"
    Write-Host "     compare           Compare two snapshots"
    Write-Host "     uninstall         Uninstall RustHunter from the system"
    Write-Host "     build             Build RustHunter from code (requires Docker)"
    Write-Host "     test              Perform unit and integration tests (requires Docker)"
    Write-Host "     help              This help"
    Write-Host
    Write-Host "ARGS:"
    Write-Host
    Write-Host "usage: $0 install"
    Write-Host
    Write-Host "usage: $0 list"
    Write-Host
    Write-Host "usage: $0 local -ConfigFile CONFIG_FILE"
    Write-Host
    Write-Host "     -ConfigFile         Configuration file"
    Write-Host
    Write-Host "usage: $0 global -HostsFile HOSTS_FILE -ConfigFile CONFIG_FILE"
    Write-Host
    Write-Host "     -HostsFile          Hosts file"
    Write-Host "     -ConfigFile         Configuration file"
    Write-Host
    Write-Host "usage: $0 compare -InitialSnapshot INITIAL_SNAPSHOT -CurrentSnapshot CURRENT_SNAPSHOT"
    Write-Host
    Write-Host "     -InitialSnapshot    Initial snapshot"
    Write-Host "     -CurrentSnapshot    Current snapshot"
    Write-Host
    Write-Host "usage: $0 uninstall"
    Write-Host
    Write-Host "usage: $0 build"
    Write-Host
    Write-Host "usage: $0 test -UnitTests -IntegrationTests -ValidationTests"
    Write-Host
    Write-Host "     -UnitTests          Perform unit tests"
    Write-Host "     -IntegrationTests   Perform integration tests"
    Write-Host "     -ValidationTests    Perform validation tests"
    Write-Host

}

function CheckInstallation {
    if ( !(Test-Path ${INSTALLATION_PATH}\rusthunter.exe) ){
        Write-Host " [*] The tool has not been installed yet" -ForegroundColor red        
        Exit 1
    }
}

function CheckDocker {
    if ( !$(docker --version) ){
        Write-Host " [*] Please install Docker Desktop for Windows" -ForegroundColor yellows        
        Exit 1
    }
}

function Build-BuilderImage {
    Write-Host " [+] Building builder docker image" -ForegroundColor green
    docker build -t ${BUILDER_IMAGE_NAME} ${BUILDER_IMAGE_PATH}
}

function Build-LauncherImage {
    Write-Host " [+] Building launcher docker image" -ForegroundColor green
    docker build -t ${LAUNCHER_IMAGE_NAME} ${LAUNCHER_IMAGE_PATH}
}

function Install-RustHunter {
    if ( !(Test-Path ${WINDOWS_BINARIES_PATH}\${EXECUTABLE_NAME}) ){
        Write-Host " [*] The tool has not been built yet" -ForegroundColor red        
        Exit 1
    } else {
        Write-Host " [+] Installing executable" -ForegroundColor green
        cp ${WINDOWS_BINARIES_PATH}\${EXECUTABLE_NAME} ${INSTALLATION_PATH}
    }

    Write-Host " [+] Successfully installed" -ForegroundColor green
}

function Show-Plugins {
    CheckInstallation
    
    rusthunter.exe list
}

function Get-LocalSnapshot {
    CheckInstallation

    if (!$ConfigFile) {
        Write-Host " [!] Please specify a config file" -ForegroundColor yellow
        Exit 1
    }

    Write-Host " [+] Creating snapshots directory" -ForegroundColor green
    mkdir -p ${SNAPSHOT_PATH} > $null

    Write-Host " [+] Collecting data" -ForegroundColor green
    rusthunter.exe run -c $ConfigFile -b ${WINDOWS_BINARIES_PATH}
    mv snapshot.json ${SNAPSHOT_PATH}

    Write-Host " [+] Merging data" -ForegroundColor green
    rusthunter.exe merge -d ${SNAPSHOT_PATH}

    Write-Host " [*] Cleaning up" -ForegroundColor green
    Remove-Item -Force -Recurse ${SNAPSHOT_PATH}
}

function Get-GlobalSnapshot {
    CheckInstallation

    if (! $ConfigFile) {
        Write-Host " [!] Please specify a config file" -ForegroundColor yellow
        Exit 1
    }

    if (! $ConfigFile) {
        Write-Host " [!] Please specify an hosts file" -ForegroundColor yellow
        Exit 1
    }

    CheckDocker

    Build-LauncherImage

    cp $HostsFile ${ANSIBLE_PATH}/${DEFAULT_HOSTS_FILE}
    cp $ConfigFile ${LINUX_BINARIES_PATH}/${DEFAULT_CONFIG_FILE}
    cp $ConfigFile ${WINDOWS_BINARIES_PATH}/${DEFAULT_CONFIG_FILE}

    Write-Host " [+] Creating snapshots directory" -ForegroundColor green
    mkdir -p ${SNAPSHOT_PATH} > $null

    Write-Host " [+] Collecting data" -ForegroundColor green
    docker run --rm -v $PWD\${ANSIBLE_PATH}:/etc/ansible -v $PWD\${SNAPSHOT_PATH}:/snapshots -w /etc/ansible -it ${LAUNCHER_IMAGE_NAME}:latest ansible-playbook playbook.yml

    Write-Host " [+] Merging data" -ForegroundColor green
    rusthunter.exe merge -d ${SNAPSHOT_PATH}

    Write-Host " [*] Cleaning up" -ForegroundColor green
    Remove-Item -Force -Recurse ${SNAPSHOT_PATH}
}

function Compare-Snapshots {
    CheckInstallation

    if (! $InitialSnapshot) {
        Write-Host " [!] Please specify an initial snapshot file" -ForegroundColor yellow
        Exit 1
    }

    if (! $CurrentSnapshot) {
        Write-Host " [!] Please specify a current snapshot file" -ForegroundColor yellow
        Exit 1
    }

    rusthunter.exe compare -i ${InitialSnapshot} -c ${CurrentSnapshot}
}

function Uninstall-RustHunter {
    CheckInstallation

    if ( $(docker --version) ){
        Write-Host " [-] Removing docker images" -ForegroundColor yellow
        docker rmi ${BUILDER_IMAGE_NAME} ${LAUNCHER_IMAGE_NAME} --force
    }

    Write-Host " [-] Removing executable" -ForegroundColor yellow
    rm -f ${INSTALLATION_PATH}\rusthunter.exe
}

function Build-RustHunter {
    CheckDocker

    Build-BuilderImage

    Build-LauncherImage

    Write-Host " [+] Building release for Linux target" -ForegroundColor green
    docker container run --rm -v ${PWD}\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo build --target x86_64-unknown-linux-gnu --release

    Write-Host " [+] Building release for Windows target" -ForegroundColor green
    docker container run --rm -v ${PWD}\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo build --target x86_64-pc-windows-msvc --release

    Write-Host " [+] Moving executables to the launcher folders" -ForegroundColor green
    cp ${APP_PATH}\target\x86_64-unknown-linux-gnu\release\rusthunter ${LINUX_BINARIES_PATH}
    cp ${APP_PATH}\target\x86_64-pc-windows-msvc\release\rusthunter.exe ${WINDOWS_BINARIES_PATH}

    Write-Host " [+] Installing executable" -ForegroundColor green
    cp ${APP_PATH}\target\x86_64-pc-windows-msvc\release\rusthunter.exe ${INSTALLATION_PATH}

    Write-Host " [*] Cleaning up" -ForegroundColor green
    Remove-Item -Force -Recurse ${APP_PATH}\target
}

function Test-RustHunter {
    CheckInstallation

    if ( !$UnitTests -and !$IntegrationTests -and !$ValidationTests ) {
        Write-Host " [*] No tests specified" -ForegroundColor yellow
        Exit 1
    }

    CheckDocker

    Build-LauncherImage

    if ( $UnitTests ) {
        Write-Host " [*] Unit testing for Linux target" -ForegroundColor green
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --lib --target x86_64-unknown-linux-gnu

        Write-Host " [*] Unit testing for Windows target" -ForegroundColor green
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --lib --target x86_64-pc-windows-msvc
    }
    
    if ( $IntegrationTests ) {
        Write-Host " [*] Integration testing for Linux target" -ForegroundColor green
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --test integration
    }

    if ( $ValidationTests ) {
        Write-Host " [*] Creating snapshots directory" -ForegroundColor green
        mkdir -p ${SNAPSHOT_PATH} > $null

        Write-Host " [*] Creating target dockers" -ForegroundColor green
        docker network create rusthunter_test_net --driver=bridge --subnet="192.168.100.1/24"
        for ($i = 2 ; $i -le 20 ; $i++) {
            docker run --network=rusthunter_test_net --ip="192.168.100.$i" -d ghcr.io/s1ntaxe770r/image:latest
        }

        Write-Host " [*] Collecting data" -ForegroundColor green
        docker run --rm -v $PWD\${ANSIBLE_PATH}:/etc/ansible -v $PWD\${SNAPSHOT_PATH}:/snapshots -w /etc/ansible --network=rusthunter_test_net ${LAUNCHER_IMAGE_NAME}:latest ansible-playbook playbook.yml -i hosts.test

        Write-Host " [*] Merging data" -ForegroundColor green
        rusthunter merge -d ${SNAPSHOT_PATH}

        Write-Host " [*] Cleaning up" -ForegroundColor green
        docker rm $(docker network inspect rusthunter_test_net --format='{{range $id, $_ := .Containers}}{{println $id}}{{end}}') --force
        docker network rm rusthunter_test_net
        Remove-Item -Force -Recurse ${SNAPSHOT_PATH}
    }
}

# Setup error handling.
Trap {
    $_
    Exit 1
}
$ErrorActionPreference = "Stop"

ShowBanner

switch ($Subcommand) {
    "install"   { Install-RustHunter }
    "list"      { Show-Plugins }
    "local"     { Get-LocalSnapshot }
    "global"    { Get-GlobalSnapshot }
    "compare"   { Compare-Snapshots }
    "uninstall" { Uninstall-RustHunter }
    "build"     { Build-RustHunter }
    "test"      { Test-RustHunter } 
    default     { ShowHelp }
}
