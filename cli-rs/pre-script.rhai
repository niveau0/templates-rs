let context = variable::prompt("Bounded context name for generated code: ", "example");
variable::set("context", context);

let path = "src/ctx_" + context;
file::rename("src/ctx_example", "src/ctx_" + context);

let cmd = variable::prompt("Command name: ", "cmd");
variable::set("cmd", cmd);
variable::set("Cmd", to_upper_camel_case(cmd));

let subcmd = variable::prompt("Subcommand name: ", "none");
variable::set("subcmd", subcmd);
variable::set("Subcmd", to_upper_camel_case(subcmd));

file::rename(path + "/cmd.rs", path + "/" + cmd + ".rs");
