mod cfg;
mod command;
mod constants;
mod ctx_example;
mod error;

use crate::constants::AUTHORS;
use crate::constants::VERSION;
use clap::{Arg, ArgMatches};
use command::Command;

pub fn version() -> &'static str {
    VERSION
}

pub fn authors() -> &'static str {
    AUTHORS
}

{% if async != "none" %}
pub async fn run() -> Result<(), error::Error> {
{% else %}
pub fn run() -> Result<(), error::Error> {
{% endif %}
    let commands = configure_commands();
    let arg_matches = parse_cli(&commands);
    init_log(&arg_matches);

    let maybe_filename = arg_matches.value_of("config");
    let config = cfg::read_config(&maybe_filename).unwrap();

    commands.iter().find_map(|cmd| cmd.execute_on_match(&config, &arg_matches))
        .expect("Unexpected error: command not found")
}


/// Configure command instances and provide a map from command identifier to command instance.
///
/// # Arguments
///
/// * `config` - configuration
fn configure_commands() -> Vec<Box<dyn Command>> {
    let mut commands = vec![];

    ctx_example::commands().into_iter().for_each(|v| commands.push(v));
    commands
}

fn init_log(matches: &ArgMatches) {
    let loglevel = match matches.occurrences_of("v") {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    };

    let loglevel = match matches.value_of("module") {
        Some(module) => {
            let mut module_loglevel = String::from(module);
            module_loglevel.push('=');
            module_loglevel.push_str(loglevel);
            module_loglevel
        }
        _ => String::from(loglevel),
    };

    std::env::set_var("RUST_LOG", &loglevel);
    env_logger::init();
    log::debug!("Setting log level to {}", &loglevel);
}

fn parse_cli(commands: &[Box<dyn Command>]) -> clap::ArgMatches {
    let parser = clap::Command::new("{{crate_name}}")
        .version(version())
        .author(authors())
        .about("This is an example commandline interface")
        .arg_required_else_help(true)
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Sets the configuration file name")
            .takes_value(true))
        .arg(Arg::new("module")
            .short('m')
            .long("module")
            .takes_value(true)
            .help("Sets the optional name of the module for which to set the verbosity level"))
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .help("Level of verbosity (error is default) if used multiple times: warn(v), info(vv), debug(vvv) and trace(vvvv)"));

    commands
        .iter()
        .fold(parser, |parser, cmd| parser.subcommand(cmd.configuration()))
        .get_matches()
}
