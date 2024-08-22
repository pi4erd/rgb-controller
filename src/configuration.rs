use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct ConfigurationError(pub &'static str);

impl Error for ConfigurationError {}
impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigurationError: {}", self.0)
    }
}

pub const CURRENT_FORMAT_VERSION: usize = 2;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ControllerConfig {
    #[serde(default)]
    pub controller_id: usize,
    #[serde(default)]
    pub selected_mode: usize,
    #[serde(default)]
    pub function_config: HashMap<String, toml::Value>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormatInfo {
    #[serde(default = "current_version")]
    pub version: usize,
}

impl Default for FormatInfo {
    fn default() -> Self {
        Self {
            version: current_version(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Configuration {
    pub format_info: FormatInfo,
    pub controller_configs: HashMap<String, ControllerConfig>,
}

fn current_version() -> usize {
    CURRENT_FORMAT_VERSION
}
