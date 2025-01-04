pub mod wit {
  wit_bindgen::generate!({
    world: "octahive:octabot/octabot",
    path: "../../wit",
    pub_export_macro: true,
    export_macro_name: "export",
    generate_all,
  });
}

pub use wit::{
  export,
  exports::octahive::octabot::plugin::{ActionData, Error, Guest as Plugin, Metadata, PluginResult, TaskData},
};
