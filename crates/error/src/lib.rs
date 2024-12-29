use bindings::wit::exports::octahive::octabot::plugin::Error as WitError;

#[derive(thiserror::Error, Debug)]
pub enum PluginError {
  #[error("Unexpected error: {0}")]
  Other(String),
}

impl From<PluginError> for WitError {
  fn from(error: PluginError) -> WitError {
    match error {
      PluginError::Other(s) => WitError::Other(s),
    }
  }
}
