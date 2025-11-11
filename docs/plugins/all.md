# All plugins

| Plugin | Description | Operating System | MITRE ATT&CK |
| ---- | ---- |  ---- | ---- |
| [linux_bashrc](linux-plugins/bashrc) | Shell configuration files for persistence detection | Linux | T1546.004 |
| [linux_crontab](linux-plugins/crontab) | Crontab jobs | Linux | T1053.003 |
| [linux_dns](linux-plugins/dns) | DNS in use | Linux | T1071.004 |
| [linux_guid](linux-plugins/guid) | Files with setgid permission | Linux | T1548.001 |
| [linux_kernel_modules](linux-plugins/kernel_modules) | Loaded kernel modules | Linux | T1082 |
| [linux_ld_preload](linux-plugins/ld_preload) | LD_PRELOAD configurations for library hijacking detection | Linux | T1574.006 |
| [linux_network_connections](linux-plugins/network_connections) | Active network connections (TCP/UDP) | Linux | T1049 |
| [linux_promisc](linux-plugins/promisc) | Network interfaces in promiscuous mode | Linux | T1040 |
| [linux_root](linux-plugins/root) | Local root users | Linux | T1136.001 |
| [linux_running_processes](linux-plugins/running_processes) | Running processes with full command line | Linux | T1057 |
| [linux_ssh_keys](linux-plugins/ssh_keys) | SSH authorized keys for all users | Linux | T1098.004 |
| [linux_suid](linux-plugins/suid) | Files with setuid permission | Linux | T1548.001 |
| [linux_systemd_timers](linux-plugins/systemd_timers) | Active systemd timers (scheduled tasks) | Linux | T1053.006 |
| [linux_tcp_listen](linux-plugins/tcp_listen) | TCP listening ports | Linux | T1572 |
| [linux_users](linux-plugins/users) | Local users | Linux | T1136.001 |
| [windows_users](windows/users) | Local users | Windows | - |
| [windows_administrators](windows-plugins/administrators) | Local administrators | Windows | - |
| [windows_tcp_listen](windows-plugins/tcp_listen) | TCP listening ports | Windows | - |
| [windows_udp_listen](windows-plugins/udp_listen) | UDP listening ports | Windows | - |
| [windows_autoruns](windows-plugins/autoruns) | Autorun entries | Windows | - |
| [windows_yara](windows-plugins/yara) | Yara rule scanning | Windows | - |
| [windows_domain_users](windows-plugins/domain_users) | Domain users and groups | Windows | - |
| [windows_domain_computers](windows-plugins/domain_computers) | Domain computers | Windows | - |
| [windows_domain_group](windows-plugins/domain_group) | Domain group members | Windows | - |
| [macos_users](macos-plugins/users) | Local users | macOS | - |