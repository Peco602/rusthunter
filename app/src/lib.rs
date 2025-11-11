pub mod comparator;
pub mod config;
pub mod constants;
pub mod merger;
pub mod options;
pub mod plugins;
pub mod runner;
pub mod utils;
pub mod validator;

use crate::comparator::compare;
use crate::merger::merge;
use crate::options::{Mode, Options};
use crate::runner::{list, run};

use crate::plugins::Plugin;

pub fn execute(options: &Options) -> Result<(), String> {
    // Plugin list
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            use crate::plugins::windows::{
                // Import Windows plugins
                users::WindowsUsers,
                administrators::WindowsAdministrators,
                tcp_listen::WindowsTCPListen,
                udp_listen::WindowsUDPListen,
                autoruns::WindowsAutoruns,
                yara::WindowsYara,
                domain_users::WindowsDomainUsers,
                domain_computers::WindowsDomainComputers,
                domain_group::WindowsDomainGroup,
            };

            // Instantiate Windows plugins
            let windows_users = WindowsUsers::new();
            let windows_administrators = WindowsAdministrators::new();
            let windows_tcp_listen = WindowsTCPListen::new();
            let windows_udp_listen = WindowsUDPListen::new();
            let windows_autoruns = WindowsAutoruns::new();
            let windows_yara = WindowsYara::new();
            let windows_domain_users = WindowsDomainUsers::new();
            let windows_domain_computers = WindowsDomainComputers::new();
            let windows_domain_group = WindowsDomainGroup::new();
            let plugins: Vec<&dyn Plugin> = vec![
                                                    // Execute Windows plugins
                                                    &windows_users,
                                                    &windows_administrators,
                                                    &windows_tcp_listen,
                                                    &windows_udp_listen,
                                                    &windows_autoruns,
                                                    &windows_yara,
                                                    &windows_domain_users,
                                                    &windows_domain_computers,
                                                    &windows_domain_group,
                                                ];
         } else if #[cfg(target_os = "linux")] {
            use crate::plugins::linux::{
                // Import Linux plugins
                bashrc::LinuxBashrc,
                crontab::LinuxCrontab,
                dns::LinuxDns,
                guid::LinuxGuid,
                kernel_modules::LinuxKernelModules,
                ld_preload::LinuxLdPreload,
                network_connections::LinuxNetworkConnections,
                promisc::LinuxPromisc,
                root::LinuxRoot,
                running_processes::LinuxRunningProcesses,
                ssh_keys::LinuxSshKeys,
                suid::LinuxSuid,
                systemd_timers::LinuxSystemdTimers,
                tcp_listen::LinuxTCPListen,
                users::LinuxUsers,
            };

            // Instantiate Linux plugins
            let linux_bashrc = LinuxBashrc::new();
            let linux_crontab = LinuxCrontab::new();
            let linux_dns = LinuxDns::new();
            let linux_guid = LinuxGuid::new();
            let linux_kernel_modules = LinuxKernelModules::new();
            let linux_ld_preload = LinuxLdPreload::new();
            let linux_network_connections = LinuxNetworkConnections::new();
            let linux_promisc = LinuxPromisc::new();
            let linux_root = LinuxRoot::new();
            let linux_running_processes = LinuxRunningProcesses::new();
            let linux_ssh_keys = LinuxSshKeys::new();
            let linux_suid = LinuxSuid::new();
            let linux_systemd_timers = LinuxSystemdTimers::new();
            let linux_tcp_listen = LinuxTCPListen::new();
            let linux_users = LinuxUsers::new();
            let plugins: Vec<&dyn Plugin> = vec![
                                                    // Execute Linux plugins
                                                    &linux_bashrc,
                                                    &linux_crontab,
                                                    &linux_dns,
                                                    &linux_guid,
                                                    &linux_kernel_modules,
                                                    &linux_ld_preload,
                                                    &linux_network_connections,
                                                    &linux_promisc,
                                                    &linux_root,
                                                    &linux_running_processes,
                                                    &linux_ssh_keys,
                                                    &linux_suid,
                                                    &linux_systemd_timers,
                                                    &linux_tcp_listen,
                                                    &linux_users,
                                                ];
        } else if #[cfg(target_os = "macos")] {
            use crate::plugins::macos::{
                // Import macOS plugins
                users::MacOSUsers,
            };

            // Instantiate macOS plugins
            let macos_users = MacOSUsers::new();
            let plugins: Vec<&dyn Plugin> = vec![
                                                    // Execute macOS plugins
                                                    &macos_users,
                                                ];
        }
    }

    match options.mode {
        Mode::List => list(&plugins),
        Mode::Run => run(
            &plugins,
            &options.config,
            &options.binary_directory,
            &options.snapshot_tag,
            &options.verbose,
        ),
        Mode::Merge => merge(
            &options.merging_directory,
            &options.snapshot_tag,
            &options.verbose,
        ),
        Mode::Compare => compare(
            &options.initial_file,
            &options.current_file,
            &options.stats,
            &options.selected_host,
            &options.selected_plugin,
        ),
    }
}
