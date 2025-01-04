use log::info;

use octabot_rust_sdk::{wit::export, Error, Metadata, Plugin, PluginResult};

struct LoggingPlugin;

impl Plugin for LoggingPlugin {
  fn load() -> Metadata {
    Metadata {
      name: "Logging".to_string(),
      version: "0.1.0".to_string(),
      author: "OctaHive".to_string(),
      description: "Plugin logger usage".to_string(),
    }
  }

  fn init(_config: String) -> Result<(), Error> {
    octabot_rust_sdk::Logger::install().expect("failed to install octabot_rust_sdk::Logger");

    log::set_max_level(log::LevelFilter::Info);

    Ok(())
  }

  fn process(_payload: String) -> Result<Vec<PluginResult>, Error> {
    info!(target: "my-target", "This is good to know and has a custom target");
    Ok(vec![])
  }
}

export!(LoggingPlugin with_types_in octabot_rust_sdk::wit);
