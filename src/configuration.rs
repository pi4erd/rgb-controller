use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct ConfigurationError(pub &'static str);

impl Error for ConfigurationError {}
impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigurationError: {}", self.0)
    }
}

pub struct Configuration {
    pub controller_id: usize,
    pub selected_mode: usize,
}

impl Configuration {
    pub fn deserialize(data: &str) -> Result<Self, ConfigurationError> {
        let parsed_config = data.parse::<toml::Table>().unwrap();

        let controller_id_v = parsed_config
            .get("controller_id")
            .ok_or(ConfigurationError("'controller_id' wasn't found!"))?;
        let selected_mode_v = parsed_config
            .get("selected_mode")
            .ok_or(ConfigurationError("'selected_mode' wasn't found!"))?;

        let controller_id = match controller_id_v {
            toml::Value::Integer(v) => *v as usize,
            _ => return Err(ConfigurationError("'controller_id' wasn't an integer!")),
        };
        let selected_mode = match selected_mode_v {
            toml::Value::Integer(v) => *v as usize,
            _ => return Err(ConfigurationError("'controller_id' wasn't an integer!")),
        };

        Ok(Self {
            selected_mode,
            controller_id,
        })
    }
}
