
let context = variable::prompt("Bounded context name for generated code: ", "example");
variable::set("context", context);

let path = "src/ctx_" + context;
file::rename("src/ctx_example", "src/ctx_" + context);

//let web_framework = variable::prompt("Framework?", "axum", ["axum", "actix"]);
variable::set("webframework", "axum");
variable::set("runtime", "tokio");
