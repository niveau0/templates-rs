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
    pub http_ip: String,
    pub http_port: u16,
}

pub fn read_config(maybe_filename: &Option<&String>) -> Result<Settings, ConfigError> {
    let settings = read_merged_config(maybe_filename).map_err(|_| ConfigError::ReadConfig)?;
    let http_ip = settings.get_string("http.ip").map_err(|_| {
        ConfigError::ParseConfig("Failed to get configuration for 'http.ip'".to_owned())
    })?;
    let http_port = settings.get::<u16>("http.port").map_err(|_| {
        ConfigError::ParseConfig("Failed to get configuration for 'http.port'".to_owned())
    })?;
    Ok(Settings { http_ip, http_port })
}

pub fn read_merged_config(maybe_filename: &Option<&String>) -> Result<config::Config, ConfigError> {
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
