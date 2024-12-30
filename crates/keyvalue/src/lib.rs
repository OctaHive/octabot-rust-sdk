use bindings::wit::wasi::keyvalue::store::{self, Bucket, Error, KeyResponse};

pub struct KeyValue {
  bucket: Bucket,
}

impl KeyValue {
  pub fn open() -> Self {
    let bucket = store::open("").expect("Can't open keyvalue store");
    Self { bucket }
  }

  pub fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
    self.bucket.set(key, value)
  }

  pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
    self.bucket.get(key)
  }

  pub fn exists(&self, key: &str) -> Result<bool, Error> {
    self.bucket.exists(key)
  }

  pub fn delete(&self, key: &str) -> Result<(), Error> {
    self.bucket.delete(key)
  }

  pub fn list_keys(&self, cursor: Option<u64>) -> Result<KeyResponse, Error> {
    self.bucket.list_keys(cursor)
  }
}
