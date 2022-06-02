pub mod constants;
pub mod options;
pub mod config;
pub mod plugins;
pub mod utils;
pub mod validator;
pub mod runner;
pub mod merger;
pub mod comparator;

use crate::options::{Options, Mode};
use crate::runner::{list, run};
use crate::merger::merge;
use crate::comparator::compare;

use crate::plugins::Plugin;
use crate::plugins::linux::{
    users::LinuxUsers,
    tcp_listen::LinuxTCPListen,
    //udp_listen::LinuxUDPListen,
    };
use crate::plugins::windows::{
    users::WindowsUsers,
    administrators::WindowsAdministrators,
    tcp_listen::WindowsTCPListen,
    udp_listen::WindowsUDPListen,
    autoruns::WindowsAutoruns,
    yara::WindowsYara,
    };

pub fn execute(options: &Options) -> Result<(), String> {
    // Plugin list
    let linux_users = LinuxUsers::new();
    let linux_tcp_listen = LinuxTCPListen::new();
    //let linux_udp_listen = LinuxUDPListen::new();
    let windows_users = WindowsUsers::new();
    let windows_administrators = WindowsAdministrators::new();
    let windows_tcp_listen = WindowsTCPListen::new();
    let windows_udp_listen = WindowsUDPListen::new();
    let windows_autoruns = WindowsAutoruns::new();
    let windows_yara = WindowsYara::new();
    let plugins: Vec<&dyn Plugin> = vec![
                                            &linux_users,
                                            &linux_tcp_listen,
                                            &windows_users,
                                            &windows_administrators,
                                            &windows_tcp_listen,
                                            &windows_udp_listen,
                                            &windows_autoruns,
                                            &windows_yara,
                                        ];

    match options.mode {
        Mode::List => list(&plugins),
        Mode::Run => run(
            &plugins, 
            &options.config, 
            &options.binary_directory, 
            &options.verbose
        ),
        Mode::Merge => merge(
            &options.merging_directory, 
            &options.verbose
        ),
        Mode::Compare => compare(
            &options.initial_file, 
            &options.current_file,
            &options.stats,
            &options.selected_host,
            &options.selected_plugin
        ),
    }
}