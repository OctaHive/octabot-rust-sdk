use bindings::wit::exports::octahive::octabot::plugin::Error as WitError;

#[derive(thiserror::Error, Debug)]
pub enum PluginError {
  #[error("Parse bot config error: {0}")]
  ParseBotConfig(String),

  #[error("Parse action payload error: {0}")]
  ParseActionPaylod(String),

  #[error("Send http request error: {0}")]
  SendHttpRequest(String),

  #[error("Parse response error: {0}")]
  ParseResponse(String),

  #[error("Unexpected error: {0}")]
  Other(String),
}

impl From<PluginError> for WitError {
  fn from(error: PluginError) -> WitError {
    match error {
      PluginError::Other(s) => WitError::Other(s),
      PluginError::ParseBotConfig(e) => WitError::ParseBotConfig(e.to_string()),
      PluginError::ParseActionPaylod(e) => WitError::ParseActionPayload(e.to_string()),
      PluginError::SendHttpRequest(e) => WitError::SendHttpRequest(e.to_string()),
      PluginError::ParseResponse(e) => WitError::ParseResponse(e.to_string()),
    }
  }
}
