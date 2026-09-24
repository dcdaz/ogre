use std::path::Path;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub hostname: String,
    pub remote_path: String,
    pub local_path: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            hostname: String::new(),
            remote_path: String::new(),
            local_path: String::new(),
        }
    }
}

impl Configuration {
    pub fn new() -> Self {
        match dirs::config_dir() {
            Some(path) => {
                let full_path = format!(
                    "{}/{}",
                    path.as_os_str().to_str().unwrap(),
                    "ogre.yml"
                );
                if Path::new(&full_path).exists() {
                    Self::get_config_from_file(&full_path).unwrap()
                } else {
                    Self::default()
                }
            },
            None => Self::default()
        }
    }

    fn get_config_from_file(file_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
        .add_source(File::with_name(file_path))
        .build()?;
        config.try_deserialize::<Configuration>()
    }
}
