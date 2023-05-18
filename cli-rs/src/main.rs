extern crate log;
{% if async == "none" %}
fn main() {
    let result = {{crate_name}}::run();

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
{%- endif %}
{%- if async == "tokio" %}
#[tokio::main]
async fn main() {
    let result = {{crate_name}}::run().await;

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
{%- endif %}
{%- if async == "async_std" %}
#[async_std::main]
async fn main() {
    let result = {{crate_name}}::run().await;

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
{% endif%}
