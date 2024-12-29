use octabot_rust_sdk::{wit::export, Action, Error, Metadata, Plugin};

struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
  fn init() -> Metadata {
    Metadata {
      name: "Hello World".to_string(),
      version: "0.1.0".to_string(),
      author: "OctaHive".to_string(),
      description: "Simple plugin for octabot".to_string(),
    }
  }

  fn process(_config: String, _payload: String) -> Result<Vec<Action>, Error> {
    println!("Hello, world!");
    Ok(vec![])
  }
}

export!(HelloWorldPlugin with_types_in octabot_rust_sdk::wit);
