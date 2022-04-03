use crate::cfg::CliConfig;
use crate::command::Command;
use crate::command::CommandRunner;
use crate::error;
use clap::ArgMatches;

pub struct Example {}

impl Example {
    pub fn new() -> Self {
        Example {}
    }
}

fn example(_config: &CliConfig, _args: &ArgMatches) -> Result<(), error::Error> {
    println!("Example command execution");
    Ok(())
}

fn subcmd(_config: &CliConfig, _args: &ArgMatches) -> Result<(), error::Error> {
    println!("Example subcmd command execution");
    Ok(())
}

impl Command for Example {
    fn matches(&self, cmd: &ArgMatches) -> Option<CommandRunner> {
        match cmd.subcommand() {
            Some(("example", args)) => match args.subcommand() {
                Some(("subcmd", _args)) => Some(subcmd),
                _ => Some(example),
            },
            _ => None,
        }
    }

    fn cli(&self) -> clap::Command<'static> {
        clap::Command::new("example")
            .arg(clap::Arg::new("a").long("arg").help("Example argument"))
            .subcommand(
                clap::Command::new("subcmd")
                    .arg(
                        clap::Arg::new("sa")
                            .required(true)
                            .short('s')
                            .long("subcmdarg")
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
        let command = Example::new();
        let parser = clap::Command::new("test").subcommand(command.cli());
        let arg_matches = parser.get_matches_from(vec!["test", "example"]);
        let maybe_runner = command.matches(&arg_matches);
        assert!(maybe_runner.is_some())
    }

    #[test]
    pub fn test_command_not_matching() {
        let command = Example::new();
        let maybe_runner = command.matches(&ArgMatches::default());
        assert!(maybe_runner.is_none())
    }
}
