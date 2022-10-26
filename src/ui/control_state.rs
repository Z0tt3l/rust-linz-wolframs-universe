use crate::config::Config;

pub struct ControlState {
    pub enabled: bool,
    pub step_size: usize,
}

impl From<&Config> for ControlState {
    fn from(config: &Config) -> Self {
        Self {
            enabled: config.enabled,
            step_size: config.step_size,
        }
    }
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            enabled: true,
            step_size: 1,
        }
    }
}
