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
    $SnapshotTag = $null,

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
${MACOS_BINARIES_PATH}="${ANSIBLE_PATH}\roles\macos\files"
${WINDOWS_BINARIES_PATH}="${ANSIBLE_PATH}\roles\windows\files"
${SNAPSHOT_PATH}=".\launcher\snapshots"

${DEFAULT_CONFIG_FILE}="config.ini"
${DEFAULT_HOSTS_FILE}="hosts"
###########################################################

function ShowBanner {
    cls
    Write-Host "  /#######                        /##     /##   /##                       /##                          " -ForegroundColor Blue
    Write-Host " | ##__  ##                      | ##    | ##  | ##                      | ##                          " -ForegroundColor Blue
    Write-Host " | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   " -ForegroundColor Blue
    Write-Host " | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  " -ForegroundColor Blue
    Write-Host " | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  " -ForegroundColor Blue
    Write-Host " | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        " -ForegroundColor Blue
    Write-Host " | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        " -ForegroundColor Blue
    Write-Host " |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        " -ForegroundColor Blue
    Write-Host ""
}

function Show-Help {
    Write-Host "usage: $0 [SUBCOMMAND] [ARGS]"
    Write-Host
    Write-Host "SUBCOMMANDS:"
    Write-Host
    Write-Host "     install             Install RustHunter on the system"
    Write-Host "     list                List all available plugins"
    Write-Host "     hosts               Protect the hosts inventory file"
    Write-Host "     local               Take a local snapshot"
    Write-Host "     global              Take a global snapshot based on hosts file (requires Docker)"
    Write-Host "     compare             Compare two snapshots"
    Write-Host "     uninstall           Uninstall RustHunter from the system"
    Write-Host "     build               Build RustHunter from code (requires Docker)"
    Write-Host "     update              Get the latest RustHunter updates"
    Write-Host "     help                This help"
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
    Write-Host "     -SnapshotTag        Snapshot tag"
    Write-Host
    Write-Host "usage: $0 global -HostsFile HOSTS_FILE -ConfigFile CONFIG_FILE"
    Write-Host
    Write-Host "     -HostsFile          Hosts file"
    Write-Host "     -ConfigFile         Configuration file"
    Write-Host "     -SnapshotTag        Snapshot tag"
    Write-Host
    Write-Host "usage: $0 compare -InitialSnapshot INITIAL_SNAPSHOT -CurrentSnapshot CURRENT_SNAPSHOT -ShowStatistics -Host HOST -Plugin PLUGIN"
    Write-Host
    Write-Host "     -InitialSnapshot    Initial snapshot"
    Write-Host "     -CurrentSnapshot    Current snapshot"
    Write-Host "     -ShowStatistics     Show summary statistics"
    Write-Host "     -FilterHost         Filter host"
    Write-Host "     -FilterPlugin       Filter plugin"
    Write-Host

}

function Show-Error($message) {
    Write-Host " [-] ${message}" -ForegroundColor red
    Write-Host ""
    Exit 1
}

function Show-Warning($message) {
    Write-Host " [!] ${message}" -ForegroundColor yellow
}

function Show-Info($message) {
    Write-Host " [+] ${message}" -ForegroundColor green
}

function Is-ExecutableInstalled {
    if ( !(Test-Path ${INSTALLATION_PATH}\${EXECUTABLE_NAME}) ) {
        Show-Error "The tool has not been installed yet"
    }
}

function Is-DockerInstalled {
    if ( !$(docker --version 2> $null) ){
        Show-Error "Please install Docker Desktop for Windows"
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
    } else {
        Show-Info "Installing executable"
        cp ${WINDOWS_BINARIES_PATH}\${EXECUTABLE_NAME} ${INSTALLATION_PATH}
    }

    Build-LauncherImage

    Show-Info "Successfully installed"
}

function Show-Plugins {
    Is-ExecutableInstalled

    Invoke-Expression "${EXECUTABLE_NAME} list"
}

function Get-LocalSnapshot {
    Is-ExecutableInstalled

    if (!${ConfigFile}) {
      Show-Error "Please specify a config file"
    }

    Show-Info "Collecting data"
    if (!${SnapshotTag}) {
      Invoke-Expression "${EXECUTABLE_NAME} run -c ${ConfigFile} -b ${WINDOWS_BINARIES_PATH}"
    } else {
      Invoke-Expression "${EXECUTABLE_NAME} run -c ${ConfigFile} -b ${WINDOWS_BINARIES_PATH} --tag ${SnapshotTag}"
    }
}

function Is-FileEncrypted($file) {
    return $(Select-String -Path ${file} -Pattern "ANSIBLE_VAULT" -CaseSensitive) -ne $null
}

function Protect-Hosts {
    Is-DockerInstalled

    Build-LauncherImage

    if (! ${HostsFile}) {
        Show-Error "Please specify an hosts file"
    }

    [array]$choices = @(${EncryptHosts}, ${RekeyHosts}, ${ViewHosts}, ${EditHosts}, ${DecryptHosts}) | Where-Object {$_ -ne $false}

    if ( ${choices}.count -eq 0 ) {
        Show-Error "Please specify an action on the hosts inventory file"
    }
    if ( ${choices}.count -gt 1) {
        Show-Error "Please specify only one action on the hosts inventory file"
    }

    $isFileEncrypted = Is-FileEncrypted ${HostsFile}

    if ( ${EncryptHosts} -and ${isFileEncrypted} ) {
        Show-Error "${HostsFile} is already encrypted"
    }

    if ( ( ${RekeyHosts} -or ${ViewHosts} -or ${EditHosts} -or ${DecryptHosts}) -and !${isFileEncrypted} ) {
        Show-Error "${HostsFile} is not encrypted"
    }

    if ( ${EncryptHosts} -and !${isFileEncrypted} ) {
        ${Command} = "encrypt"
        Show-Info "Encrypting hosts file"
    } elseif ( ${RekeyHosts} ) {
        ${Command} = "rekey"
        Show-Info "Rekeying hosts file"
    } elseif ( ${ViewHosts} ) {
        ${Command} = "view"
        Show-Info "Showing hosts file"
    } elseif ( ${EditHosts} ) {
        ${Command} = "edit"
        Show-Info "Editing hosts file"
    } elseif ( ${DecryptHosts} ) {
        ${Command} = "decrypt"
        Show-Info "Decrypting hosts file"
    }

    docker run --rm -v $PWD\${HostsFile}:/tmp/hosts -it ${LAUNCHER_IMAGE_NAME}:latest bash -c "cp /tmp/hosts /tmp/host.tmp;env EDITOR=nano ansible-vault ${Command} /tmp/host.tmp; cp /tmp/host.tmp /tmp/hosts"
}

function Get-GlobalSnapshot {
    Is-ExecutableInstalled

    Is-DockerInstalled

    Build-LauncherImage

    if (! ${ConfigFile}) {
        Show-Error "Please specify a config file"
    }

    if (! ${HostsFile}) {
        Show-Error "Please specify an hosts file"
    }

    cp ${HostsFile} ${ANSIBLE_PATH}/${DEFAULT_HOSTS_FILE}
    cp ${ConfigFile} ${LINUX_BINARIES_PATH}/${DEFAULT_CONFIG_FILE}
    cp ${ConfigFile} ${MACOS_BINARIES_PATH}/${DEFAULT_CONFIG_FILE}
    cp ${ConfigFile} ${WINDOWS_BINARIES_PATH}/${DEFAULT_CONFIG_FILE}

    Show-Info "Creating snapshots directory"
    mkdir -Force -Path ${SNAPSHOT_PATH} > $null

    Show-Info "Collecting data"
    ${Arguments} = ""
    if ( Is-FileEncrypted ${HostsFile} ) {
      ${Arguments} += " --ask-vault-pass"
    }

    if (${SnapshotTag}) {
      ${Arguments} += " --extra-vars `"snapshot_tag=${SnapshotTag}`""
    }

    Invoke-Expression "docker run --rm -v $PWD\${ANSIBLE_PATH}:/etc/ansible -v $PWD\${SNAPSHOT_PATH}:/snapshots -w /etc/ansible -it ${LAUNCHER_IMAGE_NAME}:latest ansible-playbook ${Arguments} playbook.yml"

    Show-Info "Merging data"
    ${Arguments} = "-d ${SNAPSHOT_PATH}"

    if ( ${SnapshotTag}) {
      Invoke-Expression "${EXECUTABLE_NAME} merge -d ${SNAPSHOT_PATH} --tag ${SnapshotTag}"
    } else {
      Invoke-Expression "${EXECUTABLE_NAME} merge -d ${SNAPSHOT_PATH}"
    }

    Show-Info "Cleaning up"
    Remove-Item -Force -Recurse ${SNAPSHOT_PATH}
}

function Compare-Snapshots {
    Is-ExecutableInstalled

    ${Arguments} = ""

    if ( !${InitialSnapshot} ) {
        Show-Error "Please specify an initial snapshot file"
    } else {
        ${Arguments} += " --initial ${InitialSnapshot}"
    }

    if ( !${CurrentSnapshot} ) {
        Show-Error "Please specify a current snapshot file"
    } else {
        ${Arguments} += " --current ${CurrentSnapshot}"
    }

    if ( ${FilterPlugin} -and !${FilterHost}) {
        Show-Error "Please filter also by host"
    }

    if ( ${FilterHost} ) {
        ${Arguments} += " --host ${FilterHost}"
    }

    if ( ${FilterPlugin} ) {
        ${Arguments} += " --plugin ${FilterPlugin}"
    }

    if ( ${ShowStatistics} ) {
        ${Arguments} += " --stats"
    }

    Invoke-Expression "${EXECUTABLE_NAME} compare ${Arguments}"
}

function Uninstall-RustHunter {
    Is-ExecutableInstalled

    if ( $(docker --version) ){
        Show-Warning "Removing docker images"
        docker rmi ${BUILDER_IMAGE_NAME} ${LAUNCHER_IMAGE_NAME} --force
    }

    Show-Warning "Removing executable"
    Remove-Item -Force ${INSTALLATION_PATH}\${EXECUTABLE_NAME}
}

function Build-RustHunter {
    Is-DockerInstalled

    Build-BuilderImage

    Show-Info "Building release for Linux target"
    docker container run --rm -v ${PWD}\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo build --target x86_64-unknown-linux-gnu --release

    Show-Info "Building release for macOS target"
    docker container run --rm -v ${PWD}\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo build --target x86_64-apple-darwin --release

    Show-Info "Building release for Windows target"
    docker container run --rm -v ${PWD}\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo build --target x86_64-pc-windows-msvc --release

    Show-Info "Moving executables to the launcher folders"
    cp ${APP_PATH}\target\x86_64-unknown-linux-gnu\release\rusthunter ${LINUX_BINARIES_PATH}
    cp ${APP_PATH}\target\x86_64-apple-darwin\release\rusthunter ${MACOS_BINARIES_PATH}
    cp ${APP_PATH}\target\x86_64-pc-windows-msvc\release\rusthunter.exe ${WINDOWS_BINARIES_PATH}

    Show-Info "Installing executable"
    cp ${APP_PATH}\target\x86_64-pc-windows-msvc\release\rusthunter.exe ${INSTALLATION_PATH}

    Show-Info "Cleaning up"
    Remove-Item -Force -Recurse ${APP_PATH}\target
}

function Update-RustHunter {
    # Requires git
    Show-Info "Downloading latest updates"
    git pull

    Show-Info "Installing new executable"
    cp ${WINDOWS_BINARIES_PATH}\${EXECUTABLE_NAME} ${INSTALLATION_PATH}

    Show-Info "Successfully updated"
}

function Test-RustHunter {
    Is-ExecutableInstalled

    Is-DockerInstalled

    if ( !${UnitTests} -and !${IntegrationTests} -and !${ValidationTests} ) {
        Show-Error "No tests specified"
    }

    if ( ${UnitTests} ) {
        Build-BuilderImage

        Show-Info "Unit testing" # OS-independent
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --lib
    }

    if ( ${IntegrationTests} ) {
        Build-BuilderImage

        Show-Info "Integration testing" # OS-dependent
        docker run --rm -v $PWD\${APP_PATH}:/app -w /app ${BUILDER_IMAGE_NAME}:latest cargo test --test integration
    }

    if ( $ValidationTests ) {
        Show-Info "Creating target dockers"
        $N=4
        echo "[linux]" | Out-File -Encoding ASCII test.hosts
        for ($i = 2 ; $i -le $N ; $i++) {
            ${TARGET_NAME}="target-$i"
            docker run --name ${TARGET_NAME} -d peco602/ssh-linux-docker:latest
            ${TARGET_IP}=$(docker inspect -f "{{ .NetworkSettings.Networks.bridge.IPAddress }}" $TARGET_NAME)
            echo "${TARGET_IP} ansible_connection=ssh ansible_user=user ansible_ssh_password=Pa`$`$w0rd123! ansible_become_password=Pa`$`$w0rd123!" | Out-File -Encoding ASCII -Append test.hosts
        }

        ${ConfigFile} = ${DEFAULT_CONFIG_FILE}
        ${HostsFile} = "test.hosts"
        ${SnapshotTag} = "validation"

        Get-GlobalSnapshot

        Show-Info "Destroying target dockers"
        for ($i = 2 ; $i -le $N ; $i++) {
            ${TARGET_NAME}="target-$i"
            docker rm $TARGET_NAME --force
        }
        Remove-Item ${HostsFile}
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
    "update"    { Update-RustHunter }
    "test"      { Test-RustHunter }
    default     { Show-Help }
}
