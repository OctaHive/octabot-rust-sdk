use bindings::wit::wasi::keyvalue::store::{self, Bucket, KeyResponse};
use error::PluginError;

pub struct KeyValue {
  bucket: Bucket,
}

impl KeyValue {
  pub fn open() -> Result<Self, PluginError> {
    let bucket = store::open("").map_err(|e| PluginError::OpenStorage(e.to_string()))?;
    Ok(Self { bucket })
  }

  pub fn set(&self, key: &str, value: &[u8]) -> Result<(), PluginError> {
    self
      .bucket
      .set(key, value)
      .map_err(|e| PluginError::StorageOperation(e))
  }

  pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, PluginError> {
    self.bucket.get(key).map_err(|e| PluginError::StorageOperation(e))
  }

  pub fn exists(&self, key: &str) -> Result<bool, PluginError> {
    self.bucket.exists(key).map_err(|e| PluginError::StorageOperation(e))
  }

  pub fn delete(&self, key: &str) -> Result<(), PluginError> {
    self.bucket.delete(key).map_err(|e| PluginError::StorageOperation(e))
  }

  pub fn list_keys(&self, cursor: Option<u64>) -> Result<KeyResponse, PluginError> {
    self
      .bucket
      .list_keys(cursor)
      .map_err(|e| PluginError::StorageOperation(e))
  }
}
