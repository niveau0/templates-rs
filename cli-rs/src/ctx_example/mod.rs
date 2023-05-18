mod {{cmd}};

use crate::command::Command;

pub fn commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new({{cmd}}::{{Cmd}}::new())
    ]
}
