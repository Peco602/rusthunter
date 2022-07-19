#!/bin/bash

###########################################################
# COLORS
RESET="\e[0m"
BOLD="\e[1m"
RED="\e[31m"
GREEN="\e[32m"
YELLOW="\e[33m"
BLUE="\e[34m"
###########################################################

###########################################################
# VARIABLES
INITIAL_SNAPSHOT="NONE"
CURRENT_SNAPSHOT="NONE"
HOSTS_FILE="NONE"
ENCRYPT_HOSTS="NONE"
REKEY_HOSTS="NONE"
VIEW_HOSTS="NONE"
EDIT_HOSTS="NONE"
DECRYPT_HOSTS="NONE"
CONFIG_FILE="NONE"
SNAPSHOT_TAG="NONE"
PRINT_STATS="NONE"
FILTERED_HOST="NONE"
FILTERED_PLUGIN="NONE"
UNIT_TESTS="NONE"
INTEGRATION_TESTS="NONE"
VALIDATION_TESTS="NONE"

EXECUTABLE_NAME="rusthunter"
BUILDER_IMAGE_NAME="rusthunter/builder"
BUILDER_IMAGE_PATH="./builder"

LAUNCHER_IMAGE_NAME="rusthunter/launcher"
LAUNCHER_IMAGE_PATH="./launcher"

INSTALLATION_PATH="/usr/bin"
APP_PATH="./app"
ANSIBLE_PATH="./launcher/ansible"
LINUX_BINARIES_PATH="$ANSIBLE_PATH/roles/linux/files"
MACOS_BINARIES_PATH="$ANSIBLE_PATH/roles/macos/files"
WINDOWS_BINARIES_PATH="$ANSIBLE_PATH/roles/windows/files"
SNAPSHOT_PATH="./launcher/snapshots"

DEFAULT_CONFIG_FILE="config.ini"
DEFAULT_HOSTS_FILE="hosts"
###########################################################

function ShowBanner {
    clear
    echo -e "$BLUE$BOLD  /#######                        /##     /##   /##                       /##                          $RESET"
    echo -e "$BLUE$BOLD | ##__  ##                      | ##    | ##  | ##                      | ##                          $RESET"
    echo -e "$BLUE$BOLD | ##  \ ## /##   /##  /####### /######  | ##  | ## /##   /## /######$  /######    /######   /######   $RESET"
    echo -e "$BLUE$BOLD | #######/| ##  | ## /##_____/|_  ##_/  | ########| ##  | ##| ##__  ##|_  ##_/   /##__  ## /##__  ##  $RESET"
    echo -e "$BLUE$BOLD | ##__  ##| ##  | ##|  ######   | ##    | ##__  ##| ##  | ##| ##  \ ##  | ##    | ########| ##  \__/  $RESET"
    echo -e "$BLUE$BOLD | ##  \ ##| ##  | ## \____  ##  | ## /##| ##  | ##| ##  | ##| ##  | ##  | ## /##| ##_____/| ##        $RESET"
    echo -e "$BLUE$BOLD | ##  | ##|  ######/ /#######/  |  ####/| ##  | ##|  ######/| ##  | ##  |  ####/|  #######| ##        $RESET"
    echo -e "$BLUE$BOLD |__/  |__/ \______/ |_______/    \___/  |__/  |__/ \______/ |__/  |__/   \___/   \_______/|__/        $RESET"
    echo                                                                                           
}

function ShowHelp {
    echo "usage: $0 [SUBCOMMAND] [ARGS]"
    echo
    echo "SUBCOMMANDS:"
    echo
    echo "     install           Install RustHunter on the system"
    echo "     list              List all available plugins"
    echo "     local             Take a local snapshot"
    echo "     hosts             Protect the hosts inventory file"
    echo "     global            Take a global snapshot based on hosts file (installs Docker)"
    echo "     compare           Compare two snapshots"
    echo "     uninstall         Uninstall RustHunter from the system"
    echo "     build             Build RustHunter from code (installs Docker)"
    echo "     update            Get the latest RustHunter updates"
    echo "     help              This help"
    echo
    echo "usage: $0 hosts -HostsFile HOSTS_FILE (-h |--hosts) (-e |--encrypt) (-r |--rekey) (-v |--view) (-e |--edit) (-d |--decrypt)"
    echo
    echo "     -h |--hosts          Hosts file"
    echo "     -e |--encrypt        Add encryption"
    echo "     -r |--rekey          Change encryption key"
    echo "     -v |--view           View encrypted file"
    echo "     -e |--edit           Edit encrypted file"
    echo "     -d |--decrypt        Decrypt file"
    echo
    echo "usage: $0 local (-c|--config) CONFIG_FILE"
    echo
    echo "     -c |--config         Configuration file"
    echo
    echo "usage: $0 global (-h|--hosts) HOSTS_FILE (-c|--config) CONFIG_FILE"
    echo
    echo "     -h |--hosts          Hosts file"
    echo "     -c |--config         Configuration file"
    echo "     -t |--tag            Snapshot tag"
    echo
    echo "usage: $0 compare (-i|--initial) INITIAL_SNAPSHOT (-c|--current) CURRENT_SNAPSHOT (-s |--stats) (-h |--host) HOST (-p |--plugin) PLUGIN"
    echo
    echo "     -i |--initial         Initial snapshot"
    echo "     -c |--current         Current snapshot"
    echo "     -s |--stats           Show statistics"
    echo "     -h |--host            Filter host"
    echo "     -p |--plugin          Filter plugin"
    echo
}

###########################################################
# GENERAL FUNCTIONS
function print_error {
    echo -e "$RED$BOLD [-] $1 $RESET"
    echo
    exit 1
}

function print_warning {
    echo -e "$YELLOW$BOLD [!] $1 $RESET"
}

function print_info {
    echo -e "$GREEN$BOLD [+] $1 $RESET"
}

function is_executable_installed {
    if [ ! -f $INSTALLATION_PATH/$EXECUTABLE_NAME ]; then
        print_error "The tool has not been installed yet!"
    fi    
}

function install_docker {
    if [ ! -x "$(command -v docker)" ]; then
        print_warning "Installing docker daemon"
        apt update
        apt install -y apt-transport-https ca-certificates curl gnupg-agent software-properties-common
        curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
        curl -fsSL https://download.docker.com/linux/debian/gpg | sudo apt-key add -
        apt-key fingerprint 0EBFCD88
        add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
        add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu bionic stable"

        apt update
        apt -y install docker-ce docker-ce-cli containerd.io
        docker run hello-world
    fi
}

function build_builder_image {
    if [[ -z $(docker images -q ${BUILDER_IMAGE_NAME}:latest 2> /dev/null) ]];
    then
        print_info "Building builder docker image"
        docker build -t $BUILDER_IMAGE_NAME $BUILDER_IMAGE_PATH
    fi
}

function build_launcher_image {
    if [[ -z $(docker images -q ${LAUNCHER_IMAGE_NAME}:latest 2> /dev/null) ]];
    then
        print_info "Building launcher docker image"
        docker build -t $LAUNCHER_IMAGE_NAME $LAUNCHER_IMAGE_PATH
    fi
}

function is_file_encrypted {
    if [[ -n $(grep "ANSIBLE_VAULT" $1) ]];
    then 
        IS_FILE_ENCRYPTED="True"
    else
        IS_FILE_ENCRYPTED="False"
    fi
}

###########################################################
# SUBCOMMAND BUSINESS LOGIC

function execute_install_subcommand {
    if [ ! "$UID" -eq "0" ]; then
        print_error "Superuser privileges required"
    fi

    if [ ! -f $LINUX_BINARIES_PATH/$EXECUTABLE_NAME ]; then
        print_error "The tool has not been built yet!"
    else
        print_info "Installing"
        cp $LINUX_BINARIES_PATH/$EXECUTABLE_NAME $INSTALLATION_PATH
    fi

    build_launcher_image

    print_info "Successfully installed"
}

function execute_list_subcommand {
    is_executable_installed

    $EXECUTABLE_NAME list
}

function execute_local_subcommand {
    if [ ! "$UID" -eq "0" ]; then
        print_error "Superuser privileges required"
    fi

    is_executable_installed

    while [[ $# -gt 0 ]]; do
        key="${1}"
        case ${key} in
        -c|--config)
            CONFIG_FILE="${2}"
            shift
            shift
            ;;
        *)
            ShowHelp
            exit 1
            ;;
        esac
    done
    
    if [ "$CONFIG_FILE" == "NONE" ]; then
            print_error "Please specify the config file"
    fi

    print_info "Creating snapshots directory"
    mkdir -p $SNAPSHOT_PATH

    print_info "Collecting data"
    $EXECUTABLE_NAME run -c $CONFIG_FILE
    mv snapshot.json $SNAPSHOT_PATH

    print_info "Merging data"
    $EXECUTABLE_NAME merge -d $SNAPSHOT_PATH

    print_info "Cleaning up"
    rm -rf $SNAPSHOT_PATH
}

function execute_hosts_subcommand {
    if [ ! "$UID" -eq "0" ]; then
        print_error "Superuser privileges required"
    fi

    install_docker

    build_launcher_image

    while [[ $# -gt 0 ]]; do
        key="${1}"
        case ${key} in
        -h|--hosts)
            HOSTS_FILE=${2}
            shift
            shift
            ;;
        -e|--encrypt)
            ENCRYPT_HOSTS="True"
            shift
            shift
            ;;
        -r|--rekey)
            REKEY_HOSTS="True"
            shift
            shift
            ;;
        -v|--view)
            VIEW_HOSTS="True"
            shift
            shift
            ;;
        -t|--edit)
            EDIT_HOSTS="True"
            shift
            shift
            ;;
        -d|--decrypt)
            DECRYPT_HOSTS="True"
            shift
            shift
            ;;
        *)
            ShowHelp
            exit 1
            ;;
        esac
    done

    if [ "$HOSTS_FILE" == "NONE" ]; then
        print_error "Please specify the host file"
    fi

    if [ "$#" -lt 2 ]; then
        print_error "Please specify an action on the hosts inventory file"
    fi

    if [[ "$ENCRYPT_HOSTS" == "NONE" && "$REKEY_HOSTS" == "NONE" && "$VIEW_HOSTS" == "NONE" && "$EDIT_HOSTS" == "NONE" && "$DECRYPT_HOSTS" == "NONE" ]]; then
        print_error "Please specify only one action on the hosts inventory file"
    fi

    is_file_encrypted $HOSTS_FILE

    if [[ "$ENCRYPT_HOSTS" == "True" && "$IS_FILE_ENCRYPTED" == "True" ]]; then
        print_error "$HOSTS_FILE is already encrypted"
    fi

    if [[ ( "$REKEY_HOSTS" == "True" || "$VIEW_HOSTS" == "True" || "$EDIT_HOSTS" == "True" || "$DECRYPT_HOSTS" == "True" ) && "$IS_FILE_ENCRYPTED" == "False" ]]; then
        print_error "$HOSTS_FILE is not encrypted"
    fi

    if [[ "$ENCRYPT_HOSTS" == "True" && "$IS_FILE_ENCRYPTED" == "False" ]]; then
        COMMAND="encrypt"
        print_info "Encrypting hosts file"
    elif [ "$REKEY_HOSTS" == "True" ]; then
        COMMAND="rekey"
        print_info "Rekeying hosts file"
    elif [ "$VIEW_HOSTS" == "True" ]; then
        COMMAND="view"
        print_info "Showing hosts file"
    elif [ "$EDIT_HOSTS" == "True" ]; then
        COMMAND="edit"
        print_info "Editing hosts file"
    elif [ "$DECRYPT_HOSTS" == "True" ]; then
        COMMAND="decrypt"
        print_info "Decrypting hosts file"
    fi

    docker run --rm -v $PWD/${HOSTS_FILE}:/tmp/hosts -it ${LAUNCHER_IMAGE_NAME}:latest bash -c "cp /tmp/hosts /tmp/host.tmp;env EDITOR=nano ansible-vault $COMMAND /tmp/host.tmp; cp /tmp/host.tmp /tmp/hosts"
}

function execute_global_subcommand {
    if [ ! "$UID" -eq "0" ]; then
        print_error "Superuser privileges required"
    fi

    is_executable_installed

    install_docker

    build_launcher_image

    while [[ $# -gt 0 ]]; do
        key="${1}"
        case ${key} in
        -h|--hosts)
            HOSTS_FILE=${2}
            shift
            shift
            ;;
        -c|--config)
            CONFIG_FILE="${2}"
            shift
            shift
            ;;
        -t|--tag)
            SNAPSHOT_TAG="${2}"
            shift
            shift
            ;;
        *)
            ShowHelp
            exit 1
            ;;
        esac
    done

    if [ "$HOSTS_FILE" == "NONE" ]; then
        print_error "Please specify the host file"
    fi
    
    if [ "$CONFIG_FILE" == "NONE" ]; then
        print_error "Please specify the config file"
    fi

    cp $HOSTS_FILE $ANSIBLE_PATH/$DEFAULT_HOSTS_FILE
    cp $CONFIG_FILE $LINUX_BINARIES_PATH/$DEFAULT_CONFIG_FILE
    cp $CONFIG_FILE $MACOS_BINARIES_PATH/$DEFAULT_CONFIG_FILE
    cp $CONFIG_FILE $WINDOWS_BINARIES_PATH/$DEFAULT_CONFIG_FILE

    print_info "Creating snapshots directory"
    mkdir -p $SNAPSHOT_PATH

    print_info "Collecting data"
    docker run --rm -v $PWD/$ANSIBLE_PATH:/etc/ansible -v $PWD/$SNAPSHOT_PATH:/snapshots -w /etc/ansible -it $LAUNCHER_IMAGE_NAME:latest ansible-playbook playbook.yml

    print_info "Merging data"
    ARGS="-d $SNAPSHOT_PATH"

    if [ "$SNAPSHOT_TAG" != "NONE" ]; then
        ARGS="$ARGS --tag $SNAPSHOT_TAG"
    fi

    $EXECUTABLE_NAME merge $ARGS

    print_info "Cleaning up"
    rm -rf $SNAPSHOT_PATH
}

function execute_compare_subcommand {
    is_executable_installed

    while [[ $# -gt 0 ]]; do
        key="${1}"
        case ${key} in
        -i|--initial)
            INITIAL_SNAPSHOT=${2}
            shift
            shift
            ;;
        -c|--current)
            CURRENT_SNAPSHOT="${2}"
            shift
            shift
            ;;
        -s|--stats)
            PRINT_STATS="True"
            shift
            shift
            ;;
        -h|--host)
            FILTERED_HOST="${2}"
            shift
            shift
            ;;
        -p|--plugin)
            FILTERED_PLUGIN="${2}"
            shift
            shift
            ;;
        *)
            ShowHelp
            exit 1
            ;;
        esac
    done
    
    ARGS=""

    if [ "$INITIAL_SNAPSHOT" == "NONE" ]; then
        print_error "Please specify the initial snapshot"
    else
        ARGS="$ARGS --initial $INITIAL_SNAPSHOT"
    fi
    
    if [ "$CURRENT_SNAPSHOT" == "NONE" ]; then
        print_error "Please specify the current snapshot"
    else
        ARGS="$ARGS --current $CURRENT_SNAPSHOT"
    fi

    if [[ "$FILTERED_PLUGIN" != "NONE" && "$FILTERED_HOST" == "NONE" ]]; then
        print_error "Please filter also by host"
    fi

    if [ "$FILTERED_HOST" != "NONE" ]; then
        ARGS="$ARGS --host $FILTERED_HOST"
    fi

    if [ "$FILTERED_PLUGIN" != "NONE" ]; then
        ARGS="$ARGS --plugin $FILTERED_PLUGIN"
    fi

    if [ "$PRINT_STATS" != "NONE" ]; then
        ARGS="$ARGS --stats"
    fi

    $EXECUTABLE_NAME compare $ARGS
}

function execute_uninstall_subcommand {
    is_executable_installed

    echo -e "$YELLOW$BOLD [-] Removing executable"
    rm -f $INSTALLATION_PATH/$EXECUTABLE_NAME
 
    if [ -x "$(command -v docker)" ]; then
        echo -e "$YELLOW$BOLD [-] Removing docker images"
        docker rmi $BUILDER_IMAGE_NAME $LAUNCHER_IMAGE_NAME --force
    fi
}

function execute_build_subcommand {
    if [ ! "$UID" -eq "0" ]; then
        print_error "Superuser privileges required"
    fi

    install_docker

    build_builder_image

    print_info "Building release for Linux target"
    docker run --rm -v $PWD/$APP_PATH:/app -w /app $BUILDER_IMAGE_NAME:latest cargo build --target x86_64-unknown-linux-gnu --release

    print_info "Building release for macOS target"
    docker run --rm -v $PWD/$APP_PATH:/app -w /app $BUILDER_IMAGE_NAME:latest cargo build --target x86_64-apple-darwin --release

    print_info "Building release for Windows target"
    docker run --rm -v $PWD/$APP_PATH:/app -w /app $BUILDER_IMAGE_NAME:latest cargo build --target x86_64-pc-windows-msvc --release

    print_info "Moving executables to the launcher folders"
    cp $APP_PATH/target/x86_64-unknown-linux-gnu/release/rusthunter $LINUX_BINARIES_PATH
    cp $APP_PATH/target/x86_64-apple-darwin/release/rusthunter $MACOS_BINARIES_PATH
    cp $APP_PATH/target/x86_64-pc-windows-msvc/release/rusthunter.exe $WINDOWS_BINARIES_PATH

    print_info "Installing executable"
    cp $APP_PATH/target/x86_64-unknown-linux-gnu/release/rusthunter $INSTALLATION_PATH

    print_info "Cleaning up"
    rm -rf $APP_PATH/target
}

function execute_update_subcommand {
    print_info "Downloading latest updates"
    if [ "$(git pull)" == "Already up to date." ];
    then
        print_warning "No updates available"
    else
        print_info "Installing new executable"
        sudo cp $LINUX_BINARIES_PATH/$EXECUTABLE_NAME $INSTALLATION_PATH

        build_launcher_image

        print_info "Update successful"
    fi
}

function execute_test_subcommand {
    if [ ! "$UID" -eq "0" ]; then
        print_error "Superuser privileges required"
    fi

    is_executable_installed
    
    install_docker

    while [[ $# -gt 0 ]]; do
        key="${1}"
        case ${key} in
        -u|--unit)
            UNIT_TESTS="True"
            shift
            shift
            ;;
        -i|--intergration)
            INTEGRATION_TESTS="True"
            shift
            shift
            ;;
        -v|--validation)
            VALIDATION_TESTS="True"
            shift
            shift
            ;;
        *)
            ShowHelp
            exit 1
            ;;
        esac
    done

    if [[ "$UNIT_TESTS" == "NONE" && "$INTEGRATION_TESTS" == "NONE" && "$VALIDATION_TESTS" == "NONE" ]]; then
        print_error "No tests specified"
    fi

    if [ "$UNIT_TESTS" == "True" ]; then
        build_builder_image

        print_info "Unit testing"
        docker run --rm -v $PWD/$APP_PATH:/app -w /app $BUILDER_IMAGE_NAME:latest cargo test --lib
    fi
    
    if [ "$INTEGRATION_TESTS" == "True" ]; then
        build_builder_image
        
        print_info "Integration testing"
        docker run --rm -v $PWD/$APP_PATH:/app -w /app $BUILDER_IMAGE_NAME:latest cargo test --test integration
    fi

    if [ "$VALIDATION_TESTS" == "True" ]; then
        build_launcher_image

        cp $CONFIG_FILE $LINUX_BINARIES_PATH/$DEFAULT_CONFIG_FILE
        cp $CONFIG_FILE $MACOS_BINARIES_PATH/$DEFAULT_CONFIG_FILE
        cp $CONFIG_FILE $WINDOWS_BINARIES_PATH/$DEFAULT_CONFIG_FILE
        
        print_info "Creating snapshots directory"
        mkdir -p $SNAPSHOT_PATH

        print_info "Creating target linux dockers"
        docker network create rusthunter_test_net --driver=bridge --subnet="192.168.100.1/24"
        for i in $(seq 2 20);
        do
            docker run --network=rusthunter_test_net --ip="192.168.100.$i" -d peco602/ssh-linux-docker:latest
        done

        print_info "Collecting data"
        docker run --rm -v $PWD/$ANSIBLE_PATH:/etc/ansible -v $PWD/$SNAPSHOT_PATH:/snapshots -w /etc/ansible --network=rusthunter_test_net $LAUNCHER_IMAGE_NAME:latest ansible-playbook playbook.yml -i hosts.test

        print_info "Merging data"
        rusthunter merge -d $SNAPSHOT_PATH

        print_info "Cleaning up"
        docker rm $(sudo docker network inspect rusthunter_test_net --format='{{range $id, $_ := .Containers}}{{println $id}}{{end}}') --force
        docker network rm rusthunter_test_net
        rm -rf $SNAPSHOT_PATH
    fi
}


###########################################################
# MAIN
ShowBanner

if [[ $# -gt 0 ]]; then
    while [[ $# -gt 0 ]]; do
        key="${1}"
        case ${key} in
        install)
            shift
            execute_install_subcommand $@
            exit 0
            ;;
        list)
            shift
            execute_list_subcommand $@
            exit 0
            ;;
        local)
            shift
            execute_local_subcommand $@
            exit 0
            ;;
        hosts)
            shift
            execute_hosts_subcommand $@
            exit 0
            ;;
        global)
            shift
            execute_global_subcommand $@
            exit 0
            ;;
        compare)
            shift
            execute_compare_subcommand $@
            exit 0
            ;;
        uninstall)
            shift
            execute_uninstall_subcommand $@
            exit 0
            ;;
        build)
            shift
            execute_build_subcommand $@
            exit 0
            ;;
        update)
            shift
            execute_update_subcommand $@
            exit 0
            ;;
        test)
            shift
            execute_test_subcommand $@
            exit 0
            ;;
        help)
            ShowHelp
            exit 0
            ;;
        *)
            ShowHelp
            ;;
        esac
    done
else
    ShowHelp
fi
