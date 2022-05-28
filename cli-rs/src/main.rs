extern crate log;

/// Main executable entry method
{% if async == "tokio" %}
#[tokio::main]
{% else %}
#[async_std::main]
{% endif %}
{% if async != "none" %}
async fn main() {
{% else %}
fn main() {
{% endif %}
    let result = {{crate_name}}::run().await;

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
