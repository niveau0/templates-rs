use crate::{cfg, error};

pub trait Command {
    fn execute_on_match(&self, config: &cfg::ApplicationConfiguration, args: &clap::ArgMatches) -> Option<Result<(), error::Error>>;

    fn configuration(&self) -> clap::Command<'static>;
}
