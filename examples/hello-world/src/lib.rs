use octabot_rust_sdk::{wit::export, Error, Metadata, Plugin, PluginResult};

struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
  fn load() -> Metadata {
    Metadata {
      name: "Hello World".to_string(),
      version: "0.1.0".to_string(),
      author: "OctaHive".to_string(),
      description: "Simple plugin for octabot".to_string(),
    }
  }

  fn init(_config: String) -> Result<(), Error> {
    Ok(())
  }

  fn process(_payload: String) -> Result<Vec<PluginResult>, Error> {
    println!("Hello, world!");
    Ok(vec![])
  }
}

export!(HelloWorldPlugin with_types_in octabot_rust_sdk::wit);
