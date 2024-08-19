use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct ConfigurationError(pub &'static str);

impl Error for ConfigurationError {}
impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigurationError: {}", self.0)
    }
}

pub const CURRENT_FORMAT_VERSION: usize = 1;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ControllerConfig {
    #[serde(default)]
    pub controller_id: usize,
    #[serde(default)]
    pub selected_mode: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    #[serde(default = "current_version")]
    pub version: usize,
    pub controller_configs: HashMap<String, ControllerConfig>,
}

fn current_version() -> usize {
    CURRENT_FORMAT_VERSION
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            version: current_version(),
            controller_configs: HashMap::default(),
        }
    }
}
