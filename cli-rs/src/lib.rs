mod cfg;
mod command;
mod constants;
mod ctx_{{context}};

use crate::constants::AUTHORS;
use crate::constants::VERSION;
use clap::{Arg, ArgMatches};
use command::Command;
use std::ops::Deref;
use std::sync::Arc;
{%- if async == "tokio" %}
use tokio::sync::RwLock;
{%- endif %}
{%- if async == "async_std" %}
use async_std::sync::RwLock;
{%- endif %}
{%- if async == "none" %}
use std::sync::RwLock;
{%- endif %}

pub fn version() -> &'static str {
    VERSION
}

pub fn authors() -> &'static str {
    AUTHORS
}

pub struct Shared<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Shared<T> {
    type Target = Arc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> From<T> for Shared<T> {
    fn from(value: T) -> Self {
        Shared {
            inner: Arc::new(RwLock::new(value)),
        }
    }
}

{%- if async != "none" %}
pub async fn run() -> Result<(), String> {
{%- else %}
pub fn run() -> Result<(), String> {
{%- endif %}
    let commands = configure_commands();
    let arg_matches = parse_cli(&commands);
    init_log(&arg_matches);

    let maybe_filename = arg_matches.value_of("config");
    let settings = Shared::from(cfg::read_config(&maybe_filename).unwrap());

    for cmd in commands.iter() {
{%- if async != "none" %}
        if let Some(result) = cmd.execute_on_match(settings.clone(), &arg_matches).await {
{%- else %}
        if let Some(result) = cmd.execute_on_match(settings.clone(), &arg_matches) {
{%- endif %}
            return result.map_err(|e| e.to_string());
        }
    }
    Err("Unexpected error: command not found".to_owned())
}


/// Configure command instances and provide a map from command identifier to command instance.
///
/// # Arguments
///
/// * `config` - configuration
fn configure_commands() -> Vec<Box<dyn Command>> {
    let mut commands = vec![];

    ctx_{{context}}::commands().into_iter().for_each(|v| commands.push(v));
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
