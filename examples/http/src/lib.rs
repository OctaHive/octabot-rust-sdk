use std::time::Duration;

use octabot_rust_sdk::{wit::export, Action, Error, Metadata, Plugin, PluginError};
use serde::{Deserialize, Serialize};
use url::Url;
use waki::{Client, Method};

struct HttpPlugin;

#[derive(Serialize, Deserialize)]
struct DogAttributes {
  name: String,
  description: String,
  hypoallergenic: bool,
}

#[derive(Serialize, Deserialize)]
struct DogResult {
  id: String,
  r#type: String,
  attributes: DogAttributes,
}

#[derive(Serialize, Deserialize)]
struct DogResponse {
  data: Vec<DogResult>,
}

impl Plugin for HttpPlugin {
  fn init() -> Metadata {
    Metadata {
      name: "Http".to_string(),
      version: "0.1.0".to_string(),
      author: "OctaHive".to_string(),
      description: "Plugin http usage".to_string(),
    }
  }

  fn process(_config: String, _payload: String) -> Result<Vec<Action>, Error> {
    let url =
      Url::parse(&format!("https://dogapi.dog/api/v2/breeds")).map_err(|e| PluginError::Other(e.to_string()))?;

    let client = Client::new()
      .request(Method::Get, url.as_str())
      .connect_timeout(Duration::from_secs(60));

    let resp = match client.send() {
      Ok(resp) => match resp.status_code() {
        200 => match String::from_utf8(resp.body().unwrap()) {
          Ok(resp) => serde_json::from_str::<DogResponse>(&resp).map_err(|e| PluginError::Other(e.to_string()))?,
          Err(e) => return Err(PluginError::Other(e.to_string()).into()),
        },
        code => return Err(PluginError::Other(format!("HTTP/{}", code)).into()),
      },
      Err(e) => return Err(PluginError::Other(e.to_string()).into()),
    };

    for dog in resp.data {
      println!("{}", dog.attributes.name);
    }

    Ok(vec![])
  }
}

export!(HttpPlugin with_types_in octabot_rust_sdk::wit);
