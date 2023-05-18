use config::{self, File, FileFormat};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load config.")]
    ReadConfig,
    #[error("Configuration file {0} does not exist")]
    ConfigFileDoesNotExist(String),
    #[error("Failed to parse config. {0}")]
    ParseConfig(String),
}

#[derive(Default)]
pub struct Settings {
    pub example: String,
}

pub fn read_config(maybe_filename: &Option<&str>) -> Result<Settings, ConfigError> {
    let settings = read_merged_config(maybe_filename).map_err(|_| ConfigError::ReadConfig)?;
    let example = settings
        .get_string("example")
        .map_err(|_| ConfigError::ParseConfig("Missing key example".to_owned()))?;
    Ok(Settings { example })
}

pub fn read_merged_config(maybe_filename: &Option<&str>) -> Result<config::Config, ConfigError> {
    let mut settings = config::Config::builder();

    settings = settings.add_source(File::new("conf/defaults", FileFormat::Yaml));

    settings = match *maybe_filename {
        Some(filename) => {
            if !Path::new(filename).exists() {
                return Err(ConfigError::ConfigFileDoesNotExist(filename.to_owned()));
            } else {
                settings.add_source(File::new(filename, FileFormat::Yaml))
            }
        }
        None => settings,
    };
    settings = settings.add_source(config::Environment::with_prefix("app"));

    Ok(settings.build().unwrap())
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::cfg;

    #[test]
    fn test_read_config() {
        env::set_var("app_example2", "Hello environment");
        let settings = cfg::read_merged_config(&Some("conf/defaults.yml")).unwrap();
        assert_eq!(settings.get_string("example").unwrap(), "Hello world");
        assert_eq!(
            settings.get_string("example2").unwrap(),
            "Hello environment"
        );
    }
}
