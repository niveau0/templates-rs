use crate::cfg::ApplicationConfiguration;
use crate::command::Command;
use crate::error;
use clap::ArgMatches;

const CMD: &str = "second";
const SUBCMD: &str = "secondsub";

pub struct Second {}

impl Second {
    pub fn new() -> Self {
        Second {}
    }
}

fn maincmd(_config: &ApplicationConfiguration, _args: &ArgMatches) -> Result<(), error::Error> {
    println!("Second command execution");
    Ok(())
}

fn subcmd(_config: &ApplicationConfiguration, _args: &ArgMatches) -> Result<(), error::Error> {
    println!("Second subcmd command execution");
    Ok(())
}

impl Command for Second {
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
            .arg(clap::Arg::new("secondarg").long("arg") // command parameter
            .help("Second command example argument"))
            .subcommand(
                clap::Command::new(SUBCMD)
                    .arg(
                        clap::Arg::new("subarg") // subcommand parameter
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
        let command = Second::new();
        let parser = clap::Command::new("app").subcommand(command.configuration());
        let arg_matches = parser.get_matches_from(vec!["app", CMD]);
        let maybe_result = command.execute_on_match(&ApplicationConfiguration::default(), &arg_matches);
        assert!(maybe_result.is_some());
        maybe_result.unwrap().expect("Command test failed");
    }

    #[test]
    pub fn test_command_not_matching() {
        let command = Second::new();
        let maybe_result = command.execute_on_match(&ApplicationConfiguration::default(), &ArgMatches::default());
        assert!(maybe_result.is_none())
    }
}
