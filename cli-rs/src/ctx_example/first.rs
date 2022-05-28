use crate::cfg::ApplicationConfiguration;
use crate::command::Command;
use crate::error;
use clap::ArgMatches;

const CMD: &str = "first";
const SUBCMD: &str = "firstsub";

pub struct First {}

impl First {
    pub fn new() -> Self {
        First {}
    }
}

fn maincmd(_config: &ApplicationConfiguration, _args: &ArgMatches) -> Result<(), error::Error> {
    println!("First command execution");
    Ok(())
}

fn subcmd(_config: &ApplicationConfiguration, _args: &ArgMatches) -> Result<(), error::Error> {
    println!("Second subcmd command execution");
    Ok(())
}

impl Command for First {
    fn execute_on_match(&self, config: &ApplicationConfiguration, cmd: &ArgMatches) -> Option<Result<(), error::Error>> {
        match cmd.subcommand() {
            Some((CMD, args)) => match args.subcommand() {
                Some((SUBCMD, args)) => Some(subcmd(config, args)),
                _ => Some(maincmd(config, args)),
            },
            _ => None,
        }
    }

    fn configuration(&self) -> clap::Command<'static> {
        clap::Command::new(CMD) // cli command name
            .arg(clap::Arg::new("firstarg").long("arg").help("First command example argument"))
            .subcommand(
                clap::Command::new(SUBCMD)
                    .arg(
                        clap::Arg::new("subcmdarg")
                            .required(true)
                            .short('s')
                            .long("subarg")
                            .takes_value(true)
                            .help("Example subcommand argument"),
                    )
                    .about("Example subcommand"),
            )
            .about("Example command")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::ArgMatches;

    #[test]
    pub fn test_command_matching() {
        let command = First::new();
        let parser = clap::Command::new("app").subcommand(command.configuration());
        let arg_matches = parser.get_matches_from(vec!["app", CMD]);
        let maybe_result = command.execute_on_match(&ApplicationConfiguration::default(), &arg_matches);
        assert!(maybe_result.is_some());
        maybe_result.unwrap().expect("Command test failed");
    }

    #[test]
    pub fn test_command_not_matching() {
        let command = First::new();
        let maybe_runner = command.execute_on_match(&ApplicationConfiguration::default(), &ArgMatches::default());
        assert!(maybe_runner.is_none())
    }
}
