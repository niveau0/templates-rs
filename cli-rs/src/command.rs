use crate::{cfg::CliConfig, error};

pub type CommandRunner = fn(&CliConfig, &clap::ArgMatches) -> Result<(), error::Error>;

pub trait Command {
    fn matches(&self, args: &clap::ArgMatches) -> Option<CommandRunner>;

    fn cli(&self) -> clap::Command<'static>;
}
