use crate::error;
use config::{self, File, FileFormat};
use std::path::Path;

#[derive(Default)]
pub struct ApplicationConfiguration {
    pub example: String,
}

pub fn read_config(maybe_filename: &Option<&str>) -> Result<ApplicationConfiguration, error::Error> {
    let settings = read_merged_config(maybe_filename).map_err(|_| error::Error::ReadConfigError)?;
    let example = settings
        .get_string("example")
        .map_err(|_| error::Error::ParseConfigError("Missing key example".to_owned()))?;
    Ok(ApplicationConfiguration { example })
}

pub fn read_merged_config(maybe_filename: &Option<&str>) -> Result<config::Config, error::Error> {
    let mut settings = config::Config::builder();

    settings = settings.add_source(File::new("conf/defaults", FileFormat::Yaml));

    settings = match *maybe_filename {
        Some(filename) => {
            if !Path::new(filename).exists() {
                return Err(error::Error::ConfigFileDoesNotExistError(
                    filename.to_owned(),
                ));
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
