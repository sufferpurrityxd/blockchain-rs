use std::path::Path;
use leveldb::{
  kv::KV,
  error::Error,
  options::Options,
  database::Database,
  options::WriteOptions,
};
use leveldb::options::ReadOptions;
use serde::{
  Serialize,
  Deserialize,
};

pub struct Storage(pub Database<i32>);

impl Storage {

  pub fn new() -> Storage {
    let path = std::env::var("HOME")
      .map(|home| Path::new(&home).join(".blockchain-rs"))
      .map_err(|e| {
        log::error!("Cannot get path of block storage, err: {e:?}");
        std::process::exit(1);
      })
      .unwrap();

    let mut db_options = Options::new();
    db_options.create_if_missing = true;

    return Storage(
      Database::open(path.as_path(), db_options)
        .map_err(|e| {
          log::error!("Get error when trying to open leveldb: {e:?}");
          std::process::exit(1)
        })
        .unwrap(),
    )
  }

  pub fn add_item<'a, Item: Serialize + Deserialize<'a>>(
    &self,
    key: i32,
    item: Item
  ) -> Result<(), Error> {
    return self
      .0
      .put(
        WriteOptions::new(),
        key,
        bincode::serialize(&item)
          .unwrap()
          .as_slice(),
      );
  }

}