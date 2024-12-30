use log::info;
use octabot_rust_sdk::{wit::export, Action, Error, Metadata, Plugin};

struct KeyValuePlugin;

impl Plugin for KeyValuePlugin {
  fn init() -> Metadata {
    octabot_rust_sdk::Logger::install().expect("failed to install octabot_rust_sdk::Logger");

    log::set_max_level(log::LevelFilter::Info);

    Metadata {
      name: "KeyValue".to_string(),
      version: "0.1.0".to_string(),
      author: "OctaHive".to_string(),
      description: "Plugin keyvalue usage".to_string(),
    }
  }

  fn process(_config: String, _payload: String) -> Result<Vec<Action>, Error> {
    let keyvalue = octabot_rust_sdk::KeyValue::open();

    keyvalue.set("key", b"value").unwrap();
    let value = keyvalue.get("key").unwrap().unwrap();

    let res = String::from_utf8_lossy(&value);

    info!("{}", res);

    Ok(vec![])
  }
}

export!(KeyValuePlugin with_types_in octabot_rust_sdk::wit);
