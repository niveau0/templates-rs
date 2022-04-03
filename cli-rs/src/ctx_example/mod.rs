mod example;

use crate::command::Command;

pub fn example_command() -> impl Command {
    example::Example::new()
}
