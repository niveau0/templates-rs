extern crate log;

/// Main executable entry method
fn main() {
    let result = {{crate_name}}::run();

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
