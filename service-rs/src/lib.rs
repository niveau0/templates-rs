mod cfg;
mod constants;
mod ctx_{{context}};

use crate::constants::AUTHORS;
use crate::constants::VERSION;
use clap::{Arg, ArgMatches};
use std::ops::Deref;
use std::sync::Arc;
{%- if runtime == "tokio" %}
use tokio::sync::RwLock;
{%- endif %}
{%- if runtime == "async_std" %}
use async_std::sync::RwLock;
{%- endif %}
{%- if runtime == "none" %}
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

pub async fn run() -> Result<(), String> {
    let arg_matches = parse_cli();
    init_log(&arg_matches);

    let maybe_filename = arg_matches.get_one::<String>("config");
    let settings = Shared::from(cfg::read_config(&maybe_filename).unwrap());

    ctx_{{context}}::run_service(settings.clone()).await
}

fn init_log(matches: &ArgMatches) {
    let loglevel = match matches.get_one::<u8>("v").unwrap_or(&0) {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    };

    let loglevel = match matches.get_one::<String>("module") {
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

fn parse_cli() -> clap::ArgMatches {
    clap::Command::new("{{crate_name}}")
        .version(version())
        .author(authors())
        .about("This is an example commandline interface")
        .arg_required_else_help(true)
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .num_args(1)
            .help("Sets the configuration file name"))
        .arg(Arg::new("module")
            .short('m')
            .long("module")
            .num_args(1)
            .help("Sets the optional name of the module for which to set the verbosity level"))
        .arg(Arg::new("v")
            .short('v')
            .action(clap::ArgAction::Count)
            .help("Level of verbosity (error is default) if used multiple times: warn(v), info(vv), debug(vvv) and trace(vvvv)"))
}
