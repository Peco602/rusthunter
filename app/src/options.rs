extern crate clap;

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
    pub config: String,
    pub binary_directory: String,
    pub merging_directory: String,
    pub initial_file: String,
    pub current_file: String,
}

impl Options {
    pub fn new() -> Result<Options, String> {
      
        // https://docs.rs/clap/2.33.0/clap/index.html
        let matches = Command::new("RustHunter")
                                .version("1.0")
                                .author("Giovanni Pecoraro <giovanni1.pecoraro@protonmail.com>")
                                .about("Incident Response Tool")
                                .arg(Arg::new("verbose")
                                                .short('v')
                                                .long("verbose")
                                                .help("Enable verbose output")
                                                .takes_value(false))
                                .arg_required_else_help(true)
                                .subcommand_required(true)
                                .subcommand(Command::new("list")
                                            .about("List all the available plugins"))
                                .subcommand(Command::new("run")
                                            .about("Take a system snapshot")
                                            .arg(Arg::new("config")
                                                .short('c')
                                                .long("config")
                                                .value_name("FILE")
                                                .help("Custom config file")
                                                .takes_value(true))
                                            .arg(Arg::new("bin")
                                                .short('b')
                                                .long("binary-dir")
                                                .value_name("DIRECTORY")
                                                .help("Custom binary directory")
                                                .takes_value(true)))
                                .subcommand(Command::new("merge")
                                            .about("Merge snapshot files in a directory")
                                            .arg(Arg::new("directory")
                                                .short('d')
                                                .long("directory")
                                                .value_name("DIRECTORY")
                                                .help("Directory with snapshot files")
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
                                                .takes_value(true)))
                                .get_matches();

        let mode;
        let verbose = matches.is_present("verbose");
        let mut config = String::new();
        let mut binary_directory = String::new();
        let mut merging_directory = String::new();
        let mut initial_file = String::new();
        let mut current_file = String::new();

        match matches.subcommand() {
            Some(("list", _)) => {
                 mode = Mode::List;
            },
            Some(("run", sub_matches)) => {
                 mode = Mode::Run;
                 config = sub_matches.value_of("config").unwrap_or("config").to_string();
                 binary_directory = sub_matches.value_of("bin").unwrap_or(".").to_string();
            },
            Some(("merge", sub_matches)) => {
                 mode = Mode::Merge;
                 merging_directory = sub_matches.value_of("directory").unwrap_or("directory").to_string();
            },
            Some(("compare", sub_matches)) => {
                 mode = Mode::Compare;
                 initial_file = sub_matches.value_of("initial").unwrap_or("directory").to_string();
                 current_file = sub_matches.value_of("current").unwrap_or("directory").to_string();
            },
            _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
        };

        Ok(Options {
            mode,
            verbose,
            config,
            binary_directory,
            merging_directory,
            initial_file,
            current_file,
        })
    }
}