extern crate clap;

use crate::constants::*;

use clap::{Arg, Command};

pub enum Mode {
    Run,
    List,
    Merge,
    Compare
}

pub struct Options {
    pub mode: Mode,
    pub verbose: bool,
    // Run
    pub config: String,
    pub binary_directory: String,
    pub snapshot_tag: String,
    // Merge
    pub merging_directory: String,
    // Compare
    pub initial_file: String,
    pub current_file: String,
    pub stats: bool,
    pub selected_host: String,
    pub selected_plugin: String
}

impl Options {
    pub fn new() -> Result<Options, String> {
      
        // https://docs.rs/clap/2.33.0/clap/index.html
        let matches = Command::new("RustHunter")
                                .version(CURRENT_VERSION)
                                .author("Giovanni Pecoraro <giovanni1.pecoraro@protonmail.com>")
                                .about("Modular incident response framework to build and compare environmental baselines")
                                .arg(Arg::new("verbose")
                                                .short('v')
                                                .long("verbose")
                                                .help("Enable verbose output")
                                                .takes_value(false))
                                .arg_required_else_help(true)
                                .subcommand_required(true)
                                .subcommand(Command::new("list")
                                            .about("List available plugins"))
                                .subcommand(Command::new("run")
                                            .about("Take system snapshot")
                                            .arg(Arg::new("config")
                                                .short('c')
                                                .long("config")
                                                .value_name("FILE")
                                                .help("Config file")
                                                .takes_value(true))
                                            .arg(Arg::new("bin")
                                                .short('b')
                                                .long("binary-dir")
                                                .value_name("DIRECTORY")
                                                .help("Binary directory")
                                                .takes_value(true))
                                            .arg(Arg::new("tag")
                                                .short('t')
                                                .long("tag")
                                                .value_name("TAG")
                                                .help("Custom snapshot tag")
                                                .takes_value(true)))
                                .subcommand(Command::new("merge")
                                            .about("Merge snapshot files in a directory")
                                            .arg(Arg::new("directory")
                                                .short('d')
                                                .long("directory")
                                                .value_name("DIRECTORY")
                                                .help("Directory with snapshots to be merged")
                                                .required(true)
                                                .takes_value(true))
                                            .arg(Arg::new("tag")
                                                .short('t')
                                                .long("tag")
                                                .value_name("TAG")
                                                .help("Custom snapshot tag")
                                                .takes_value(true)))                            
                                .subcommand(Command::new("compare")
                                            .about("Compare two snapshot files")
                                            .arg(Arg::new("initial")
                                                .short('i')
                                                .long("initial")
                                                .required(true)
                                                .value_name("FILE")
                                                .help("Initial snapshot file")
                                                .takes_value(true))
                                            .arg(Arg::new("current")
                                                .short('c')
                                                .long("current")
                                                .required(true)
                                                .value_name("FILE")
                                                .help("Current snapshot file")
                                                .takes_value(true))
                                            .arg(Arg::new("stats")
                                                .short('s')
                                                .long("stats")
                                                .help("Show comparison statistics")
                                                .takes_value(false))
                                            .arg(Arg::new("host")
                                                .short('H')
                                                .long("host")
                                                .value_name("HOST")
                                                .help("Filter by host")
                                                .takes_value(true))
                                            .arg(Arg::new("plugin")
                                                .short('P')
                                                .long("plugin")
                                                .value_name("PLUGIN")
                                                .help("Filter by plugin")
                                                .takes_value(true))
                                            )
                                .get_matches();

        let mode;
        let verbose = matches.is_present("verbose");

        // Run
        let mut config = String::new();
        let mut binary_directory = String::new();
        let mut snapshot_tag = String::new();

        // Merge
        let mut merging_directory = String::new();

        // Compare
        let mut initial_file = String::new();
        let mut current_file = String::new();
        let mut stats: bool = false;
        let mut selected_host = String::new();
        let mut selected_plugin = String::new();

        match matches.subcommand() {
            Some(("list", _)) => {
                 mode = Mode::List;
            },
            Some(("run", sub_matches)) => {
                 mode = Mode::Run;
                 config = sub_matches.value_of("config").unwrap_or(DEFAULT_CONFIG_FILE).to_string();
                 binary_directory = sub_matches.value_of("bin").unwrap_or(DEFAULT_BINARY_DIR).to_string();
                 snapshot_tag = sub_matches.value_of("tag").unwrap_or(DEFAULT_SNAPSHOT_TAG).to_string();
            },
            Some(("merge", sub_matches)) => {
                 mode = Mode::Merge;
                 merging_directory = sub_matches.value_of("directory").unwrap().to_string();
                 snapshot_tag = sub_matches.value_of("tag").unwrap_or(DEFAULT_SNAPSHOT_TAG).to_string();
            },
            Some(("compare", sub_matches)) => {
                 mode = Mode::Compare;
                 initial_file = sub_matches.value_of("initial").unwrap().to_string();
                 current_file = sub_matches.value_of("current").unwrap().to_string();

                 stats = sub_matches.is_present("stats");
                 if !stats {
                    selected_host = sub_matches.value_of("host").unwrap_or("").to_string();
                    if selected_host != "" {
                        selected_plugin = sub_matches.value_of("plugin").unwrap_or("").to_string();
                    }
                 }
            },
            _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
        };

        Ok(Options {
            mode,
            verbose,
            config,
            binary_directory,
            snapshot_tag,
            merging_directory,
            initial_file,
            current_file,
            stats,
            selected_host,
            selected_plugin
        })
    }
}