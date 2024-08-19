use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct ConfigurationError(pub &'static str);

impl Error for ConfigurationError {}
impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigurationError: {}", self.0)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Configuration {
    pub controller_id: usize,
    pub selected_mode: usize,
}
