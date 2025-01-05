use log::info;
use octabot_rust_sdk::{wit::export, Error, Metadata, Plugin, PluginResult};

struct KeyValuePlugin;

impl Plugin for KeyValuePlugin {
  fn load() -> Metadata {
    Metadata {
      name: "KeyValue".to_string(),
      version: "0.1.0".to_string(),
      author: "OctaHive".to_string(),
      description: "Plugin keyvalue usage".to_string(),
    }
  }

  fn init(_config: String) -> Result<(), Error> {
    octabot_rust_sdk::Logger::install().expect("failed to install octabot_rust_sdk::Logger");

    log::set_max_level(log::LevelFilter::Info);
    Ok(())
  }

  fn process(_payload: String) -> Result<Vec<PluginResult>, Error> {
    let keyvalue = octabot_rust_sdk::KeyValue::open()?;

    keyvalue.set("key", b"value")?;

    if let Some(value) = keyvalue.get("key")? {
      let res = String::from_utf8_lossy(&value);

      info!("{}", res);
    }

    Ok(vec![])
  }
}

export!(KeyValuePlugin with_types_in octabot_rust_sdk::wit);
