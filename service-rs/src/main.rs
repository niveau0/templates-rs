extern crate log;
{% if runtime == "none" %}
fn main() {
    let result = {{crate_name}}::run();

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
{%- endif %}
{%- if runtime == "tokio" %}
#[tokio::main]
async fn main() {
    let result = {{crate_name}}::run().await;

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
{%- endif %}
{%- if runtime == "async_std" %}
#[async_std::main]
async fn main() {
    let result = {{crate_name}}::run().await;

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
{% endif%}
