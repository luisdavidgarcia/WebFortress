use crate::errors::Errcode;
use crate::config_parser::{load_config, Config};

use clap::{Arg, App};

use std::path::PathBuf;

pub fn parse_args() -> Result<Config, Errcode> {
    let matches = App::new("Crabcan")
        .version("1.0")
        .about("A Container Implmentation in Rust")
        .arg(Arg::with_name("debug")
            .short('d')
            .long("debug")
            .takes_value(false)
            .help("Activate debug mode"))
        .arg(Arg::with_name("command")
            .short('c').long("command")
            .takes_value(true)
            .help("Command to execute inside the container"))
        .arg(Arg::with_name("uid")
            .short('u')
            .long("uid")
            .takes_value(true).
            help("User ID to create inside the container"))
        .arg(Arg::with_name("mount")
            .short('m')
            .long("mount")
            .takes_value(true)
            .help("Directory to mount as root of the container"))
        .arg(Arg::with_name("add")
            .short('a').long("add")
            .takes_value(true)
            .multiple(true)
            .help("Additional paths to mount inside the container"))
        .arg(Arg::with_name("config")
            .short('f')
            .long("config")
            .takes_value(true)
            .help("Path to configuration file"))
        .get_matches();

    let config = if let Some(config_file) = matches.value_of("config") {
        load_config(PathBuf::from(config_file))?
    } else {
        Config {
            debug: matches
                .is_present("debug"),
            command: matches
                .value_of("command")
                .unwrap_or_default()
                .to_string(),
            uid: matches
                .value_of("uid")
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default(),
            mount_dir: matches
                .value_of("mount")
                .unwrap_or_default()
                .parse::<PathBuf>()
                .unwrap_or_default(),
            additional_paths: matches
                .values_of("add")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect(),
        }
    };

    setup_log(if config.debug {
        log::LevelFilter::Debug
    } else{
        log::LevelFilter::Info
    });

    Ok(config)
}

pub fn setup_log(level: log::LevelFilter){
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}