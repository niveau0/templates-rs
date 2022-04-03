#[macro_use]
extern crate log;

use clap::{Arg, ArgMatches};
use {{crate_name}}::{cfg, command::Command, ctx_example, error::Error};

/// Main executable entry method
fn main() {
    let commands = configure_commands();
    let arg_matches = parse_cli(&commands);
    init_log(&arg_matches);

    let maybe_filename = arg_matches.value_of("config");
    let config = cfg::read_config(&maybe_filename).unwrap();

    let result = match commands.iter().find_map(|cmd| cmd.matches(&arg_matches)) {
        Some(runner) => runner(&config, &arg_matches),
        _ => Err(Error::UnknownCommand),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}

/// Configure command instances and provide a map from command identifier to command instance.
///
/// # Arguments
///
/// * `config` - configuration
fn configure_commands() -> Vec<impl Command> {
    let mut commands = Vec::new();

    commands.push(ctx_example::example_command());
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
            module_loglevel.push_str("=");
            module_loglevel.push_str(loglevel);
            module_loglevel
        }
        _ => String::from(loglevel),
    };

    std::env::set_var("RUST_LOG", &loglevel);
    env_logger::init();
    debug!("Setting log level to {}", &loglevel);
}

fn parse_cli(commands: &Vec<impl Command>) -> clap::ArgMatches {
    let parser = clap::Command::new("{{crate_name}}")
        .version({{crate_name}}::version())
        .author({{crate_name}}::authors())
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
        .into_iter()
        .fold(parser, |parser, cmd| parser.subcommand(cmd.cli()))
        .get_matches()
}
