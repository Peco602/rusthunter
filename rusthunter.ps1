#Requires -RunAsAdministrator

Param(
    [Parameter(Position = 0)]
    [string]
    $Subcommand = $null,

    [string]
    $HostsFile = $null,

    [switch]
    $EncryptHosts = $null,

    [switch]
    $ViewHosts = $null,

    [switch]
    $EditHosts = $null,

    [switch]
    $RekeyHosts = $null,

    [switch]
    $DecryptHosts = $null,

    [string]
    $ConfigFile = $null,
    
    [string]
    $InitialSnapshot = $null,

    [string]
    $CurrentSnapshot = $null,

    [switch]
    $ShowStatistics = $null,

    [string]
    $FilterHost = $null,

    [string]
    $FilterPlugin = $null,

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

function Show-Help {
    Write-Host "usage: $0 [SUBCOMMAND] [ARGS]"
    Write-Host
    Write-Host "SUBCOMMANDS:"
    Write-Host
    Write-Host "     install           Install RustHunter on the system"
    Write-Host "     list              List all available plugins"
    Write-Host "     hosts             Protect the hosts inventory file"
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
    Write-Host "usage: $0 hosts -HostsFile HOSTS_FILE -EncryptHosts -RekeyHosts -ViewHosts -EditHosts -DecryptHosts"
    Write-Host
    Write-Host "     -HostsFile          Hosts file to be encrypted"
    Write-Host "     -EncryptHosts       Add encryption"
    Write-Host "     -RekeyHosts         Change encryption key"
    Write-Host "     -ViewHosts          View encrypted file"
    Write-Host "     -EditHosts          Edit encrypted file"
    Write-Host "     -DecryptHosts       Decrypt file"
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
    Write-Host "usage: $0 compare -InitialSnapshot INITIAL_SNAPSHOT -CurrentSnapshot CURRENT_SNAPSHOT -ShowStatistics -Host HOST -Plugin PLUGIN"
    Write-Host
    Write-Host "     -InitialSnapshot    Initial snapshot"
    Write-Host "     -CurrentSnapshot    Current snapshot"
    Write-Host "     -ShowStatistics     Show summary statistics"
    Write-Host "     -FilterHost         Filter host"
    Write-Host "     -FilterPlugin       Filter plugin"
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

function Show-Error($message) {
    Write-Host " [-] ${message}" -ForegroundColor red      
}

function Show-Warning($message) {
    Write-Host " [!] ${message}" -ForegroundColor yellow      
}

function Show-Info($message) {
    Write-Host " [+] ${message}" -ForegroundColor green
}

function Is-ExecutableInstalled {
    if ( !(Test-Path ${INSTALLATION_PATH}\rusthunter.exe) ) {
        Show-Error "The tool has not been installed yet"       
        Exit 1
    }
}

function Is-DockerInstalled {
    if ( !$(docker --version 2> $null) ){
        Show-Error "Please install Docker Desktop for Windows"       
        Exit 1
    }
}

function Build-BuilderImage {
    if ( !$(docker images -q ${BUILDER_IMAGE_NAME}:latest 2> $null) ) {
        Show-Info "Building builder docker image" 
        docker build -t ${BUILDER_IMAGE_NAME} ${BUILDER_IMAGE_PATH}
    }
}

function Build-LauncherImage {
    if ( !$(docker images -q ${LAUNCHER_IMAGE_NAME}:latest 2> $null) ) {
        Show-Info "Building launcher docker image"
        docker build -t ${LAUNCHER_IMAGE_NAME} ${LAUNCHER_IMAGE_PATH}
    }
}

function Install-RustHunter {
    if ( !(Test-Path ${WINDOWS_BINARIES_PATH}\${EXECUTABLE_NAME}) ){
        Show-Error "The tool has not been built yet"       
        Exit 1
    } else {
        Show-Info "Installing executable"
        cp ${WINDOWS_BINARIES_PATH}\${EXECUTABLE_NAME} ${INSTALLATION_PATH}
    }

    Show-Info "Successfully installed"
}

function Show-Plugins {
    Is-ExecutableInstalled
    
    rusthunter.exe list
}

function Get-LocalSnapshot {
    Is-ExecutableInstalled

    if (!${ConfigFile}) {
        Show-Error "Please specify a config file"
        Exit 1
    }

    Show-Info "Creating snapshots directory"
    mkdir -p ${SNAPSHOT_PATH} > $null

    Show-Info "Collecting data"
    rusthunter.exe run -c ${ConfigFile} -b ${WINDOWS_BINARIES_PATH}
    mv snapshot.json ${SNAPSHOT_PATH}

    Show-Info "Merging data"
    rusthunter.exe merge -d ${SNAPSHOT_PATH}

    Show-Info "Cleaning up"
    Remove-Item -Force -Recurse ${SNAPSHOT_PATH}
}

function Is-FileEncrypted($file) {
    return $(Select-String -Path ${file} -Pattern "ANSIBLE_VAULT" -CaseSensitive) -ne $null
}

function Protect-Hosts {
    Is-DockerInstalled

    Build-LauncherImage

    if (! ${HostsFile}) {
        Show-Error "Please specify an hosts file"
        Exit 1
    }

    [array]$choices = @(${EncryptHosts}, ${RekeyHosts}, ${ViewHosts}, ${EditHosts}, ${DecryptHosts}) | Where-Object {$_ -ne $false}

    if ( ${choices}.count -eq 0 ) {
        Show-Error "Please specify an action on the hosts inventory file"
        Exit 1
    }
    if ( ${choices}.count -gt 1) {
        Show-Error "Please specify only one action on the hosts inventory file"
        Exit 1
    }

    $isFileEncrypted = Is-FileEncrypted ${HostsFile}

    if ( ${EncryptHosts} -and ${isFileEncrypted} ) {
        Show-Error "${HostsFile} is already encrypted"
        Exit 1
    }

    if ( ( ${RekeyHosts} -or ${ViewHosts} -or ${EditHosts} -or ${DecryptHosts}) -and !${isFileEncrypted} ) {
        Show-Error "${HostsFile} is not encrypted"
        Exit 1
    }

    if ( ${EncryptHosts} -and ${isFileEncrypted} ) {
        $command = "encrypt"
        Show-Info "Encrypting hosts file"
    } elseif ( ${RekeyHosts} ) {
        $command = "rekey"
        Show-Info "Rekeying hosts file"
    } elseif ( ${ViewHosts} ) {
        $command = "view"
        Show-Info "Showing hosts file"
    } elseif ( ${EditHosts} ) {
        $command = "edit"
        Show-Info "Editing hosts file"
    } elseif ( ${DecryptHosts} ) {
        $command = "decrypt"
        Show-Info "Decrypting hosts file"
    }

    docker run --rm -v $PWD\${HostsFile}:/tmp/hosts -it ${LAUNCHER_IMAGE_NAME}:latest bash -c "cp /tmp/hosts /tmp/host.tmp;env EDITOR=nano ansible-vault $command /tmp/host.tmp; cp /tmp/host.tmp /tmp/hosts"
}

function Get-GlobalSnapshot {
    Is-ExecutableInstalled

    Is-DockerInstalled

    Build-LauncherImage

    if (! ${ConfigFile}) {
        Show-Error "Please specify a config file"
        Exit 1
    }

    if (! ${HostsFile}) {
        Show-Error "Please specify an hosts file"
        Exit 1
    }

    cp ${HostsFile} ${ANSIBLE_PATH}/${DEFAULT_HOSTS_FILE}
    cp ${ConfigFile} ${LINUX_BINARIES_PATH}/${DEFAULT_CONFIG_FILE}
    cp ${ConfigFile} ${WINDOWS_BINARIES_PATH}/${DEFAULT_CONFIG_FILE}

    Show-Info "Creating snapshots directory"
    mkdir -p ${SNAPSHOT_PATH} > $null

    Show-Info "Collecting data"
    if ( Is-FileEncrypted ${HostsFile} ) {
        docker run --rm -v $PWD\${ANSIBLE_PATH}:/etc/ansible -v $PWD\${SNAPSHOT_PATH}:/snapshots -w /etc/ansible -it ${LAUNCHER_IMAGE_NAME}:latest ansible-playbook --ask-vault-pass playbook.yml
    } else {
        docker run --rm -v $PWD\${ANSIBLE_PATH}:/etc/ansible -v $PWD\${SNAPSHOT_PATH}:/snapshots -w /etc/ansible -it ${LAUNCHER_IMAGE_NAME}:latest ansible-playbook playbook.yml
    }

    Show-Info "Merging data"
    rusthunter.exe merge -d ${SNAPSHOT_PATH}

    Show-Info "Cleaning up"
    Remove-Item -Force -Recurse ${SNAPSHOT_PATH}
}

function Compare-Snapshots {
    Is-ExecutableInstalled

    $args = ""

    if ( !${InitialSnapshot} ) {
        Show-Error "Please specify an initial snapshot file"
        Exit 1
    } else {
        $args += " --initial ${InitialSnapshot}"
    }

    if ( !${CurrentSnapshot} ) {
        Show-Error "Please specify a current snapshot file"
        Exit 1
    } else {
        $args += " --current ${CurrentSnapshot}"
    }

    if ( ${FilterPlugin} -and !${FilterHost}) {
        Show-Error "Please filter also by host"
        Exit 1
    }

    if ( ${FilterHost} ) {
        $args += " --host ${FilterHost}"
    }

    if ( ${FilterPlugin} ) {
        $args += " --plugin ${FilterPlugin}"
    }

    rusthunter.exe compare $args
}

function Uninstall-RustHunter {
    Is-ExecutableInstalled

    if ( $(docker --version) ){
        Show-Warning "Removing docker images"
        docker rmi ${BUILDER_IMAGE_NAME} ${LAUNCHER_IMAGE_NAME} --force
    }

    Show-Warning "Removing executable"
    Remove-Item -Force ${INSTALLATION_PATH}\rusthunter.exe
}

function Build-RustHunter {
    Is-DockerInstalled

    Build-BuilderImage

    Build-LauncherImage

    Show-Info "Building release for Linux target"
    docker container run --rm -v ${PWD}\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo build --target x86_64-unknown-linux-gnu --release

    Show-Info "Building release for Windows target"
    docker container run --rm -v ${PWD}\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo build --target x86_64-pc-windows-msvc --release

    Show-Info "Moving executables to the launcher folders"
    cp ${APP_PATH}\target\x86_64-unknown-linux-gnu\release\rusthunter ${LINUX_BINARIES_PATH}
    cp ${APP_PATH}\target\x86_64-pc-windows-msvc\release\rusthunter.exe ${WINDOWS_BINARIES_PATH}

    Show-Info "Installing executable"
    cp ${APP_PATH}\target\x86_64-pc-windows-msvc\release\rusthunter.exe ${INSTALLATION_PATH}

    Show-Info "Cleaning up"
    Remove-Item -Force -Recurse ${APP_PATH}\target
}

function Test-RustHunter {
    Is-ExecutableInstalled

    Is-DockerInstalled

    Build-LauncherImage

    if ( !${UnitTests} -and !${IntegrationTests} -and !${ValidationTests} ) {
        Show-Error "No tests specified"
        Exit 1
    }

    if ( ${UnitTests} ) {
        Show-Info "Unit testing for Linux target"
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --lib --target x86_64-unknown-linux-gnu

        Show-Info "Unit testing for Windows target"
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --lib --target x86_64-pc-windows-msvc
    }
    
    if ( ${IntegrationTests} ) {
        Show-Info "Integration testing for Linux target"
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --test integration
    }

    if ( $ValidationTests ) {
        Show-Info "Creating snapshots directory"
        mkdir -p ${SNAPSHOT_PATH} > $null

        Show-Info "Creating target dockers"
        docker network create rusthunter_test_net --driver=bridge --subnet="192.168.100.1/24"
        for ($i = 2 ; $i -le 20 ; $i++) {
            docker run --network=rusthunter_test_net --ip="192.168.100.$i" -d ghcr.io/s1ntaxe770r/image:latest
        }

        Show-Info "Collecting data"
        docker run --rm -v $PWD\${ANSIBLE_PATH}:/etc/ansible -v $PWD\${SNAPSHOT_PATH}:/snapshots -w /etc/ansible --network=rusthunter_test_net ${LAUNCHER_IMAGE_NAME}:latest ansible-playbook playbook.yml -i hosts.test

        Show-Info "Merging data"
        rusthunter merge -d ${SNAPSHOT_PATH}

        Show-Info "Cleaning up"
        docker rm $(docker network inspect rusthunter_test_net --format='{{range $id, $_ := .Containers}}{{println $id}}{{end}}') --force
        docker network rm rusthunter_test_net
        Remove-Item -Force -Recurse ${SNAPSHOT_PATH}
    }
}

# Setup error handling
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
    "hosts"     { Protect-Hosts }
    "global"    { Get-GlobalSnapshot }
    "compare"   { Compare-Snapshots }
    "uninstall" { Uninstall-RustHunter }
    "build"     { Build-RustHunter }
    "test"      { Test-RustHunter } 
    default     { Show-Help }
}
