use crate::cfg::Settings;
use crate::command::{Command, CommandError};
use crate::Shared;
{%- if async != "none" %}
use async_trait::async_trait;
{%- endif %}
use clap::ArgMatches;

const CMD: &str = "{{cmd}}";
{%- if subcmd != "none" %}
const SUBCMD: &str = "{{subcmd}}";
{%- endif %}

pub struct {{Cmd}} {}

impl {{Cmd}} {
    pub fn new() -> Self {
        {{Cmd}} {}
    }
}

fn run_cmd(_settings: Shared<Settings>, _args: &ArgMatches) -> Result<(), CommandError> {
    println!("{{cmd}} command executed");
    Ok(())
}

{%- if subcmd != "none" %}
fn run_subcmd(_csettings: Shared<Settings>, _args: &ArgMatches) -> Result<(), CommandError> {
    println!("{{subcmd}} subcommand execution");
    Err(CommandError::Command("Example failure".to_owned()))
}
{%- endif %}

{%- if async != "none" %}
#[async_trait]
impl Command for {{Cmd}} {
    async fn execute_on_match(
{%- else %}
impl Command for {{Cmd}} {
    fn execute_on_match(
{%- endif %}
        &self,
        settings: Shared<Settings>,
        cmd: &ArgMatches,
    ) -> Option<Result<(), CommandError>> {
{%- if subcmd != "none" %}        
        match cmd.subcommand() {
            Some((CMD, args)) => match args.subcommand() {
                Some((SUBCMD, args)) => Some(run_subcmd(settings, args)),
                _ => Some(run_cmd(settings, args)),
            },
            _ => None,
        }
{%- else %}
        match cmd.subcommand() {
            Some((CMD, args)) => Some(run_cmd(settings, args)),
            _ => None,
        }
{%- endif %}
    }

    fn configuration(&self) -> clap::Command<'static> {
        clap::Command::new(CMD) // cli command name
            .arg(
                clap::Arg::new("first")
                    .short('f')
                    .long("first")
                    .help("{{cmd}} command example argument"),
            )
{%- if subcmd != "none" %}        
            .subcommand(
                clap::Command::new(SUBCMD)
                    .arg(
                        clap::Arg::new("second")
                            .required(true)
                            .short('s')
                            .long("second")
                            .takes_value(true)
                            .help("{{subcmd}} subcommand argument"),
                    )
                    .about("Example subcommand"),
            )
{%- endif %}
            .about("TODO Describe {{cmd}} here")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::ArgMatches;

    #[test]
    pub fn test_command_matching() {
        let command = {{Cmd}} ::new();
        let parser = clap::Command::new("app").subcommand(command.configuration());
        let arg_matches = parser.get_matches_from(vec!["app", CMD]);
        let maybe_result =
            command.execute_on_match(&ApplicationConfiguration::default(), &arg_matches);
        assert!(maybe_result.is_some());
        maybe_result.unwrap().expect("Command test failed");
    }

    #[test]
    pub fn test_command_not_matching() {
        let command = {{Cmd}} ::new();
        let maybe_runner =
            command.execute_on_match(&ApplicationConfiguration::default(), &ArgMatches::default());
        assert!(maybe_runner.is_none())
    }
}
