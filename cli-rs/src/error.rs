use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown command.")]
    UnknownCommand,
    #[error("Failed to load config.")]
    ReadConfigError,
    #[error("Configuration file {0} does not exist")]
    ConfigFileDoesNotExistError(String),
    #[error("Failed to parse config. {0}")]
    ParseConfigError(String),
}
