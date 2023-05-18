use crate::{cfg::Settings, Shared};
{%- if async != "none" %}
use async_trait::async_trait;
{%- endif %}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Failed to execute command. {0}")]
    Command(String),
}

{%- if async != "none" %}
#[async_trait]
pub trait Command {
    async fn execute_on_match(
{%- else %}
pub trait Command {
    fn execute_on_match(
{%- endif %}
        &self,
        settings: Shared<Settings>,
        args: &clap::ArgMatches,
    ) -> Option<Result<(), CommandError>>;
    
    fn configuration(&self) -> clap::Command<'static>;
}
