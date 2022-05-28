mod first;
mod second;

use crate::command::Command;

pub fn commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(first::First::new()), Box::new(second::Second::new())]
}
