use std::path::Path;
use std::fs::File;
use std::io::Write;
use crate::block::Block;

pub struct Storage {
    pub path: Box<Path>
}


impl Storage {
    pub fn new() -> Self {
        return Self {
            path: std::env::var("HOME")
                .map(|home| Path::new(&home).join(".blockchain_rs"))
                .map_err(|e| {
                    log::error!("Get error while trying to create storage: {e:?}");
                    std::process::exit(1);
                })
                .unwrap()
                .into()
        }
    }

    pub fn save_block(&self, block: &Block) {
        match serde_json::to_vec::<Block>(&block) {
            Ok(block_json) => {
                let buf = block_json.as_slice();
                let mut file = File::create(self.path.join(format!("{:?}.json",
                                                               block.hash.clone()))).unwrap();
                file.write_all(buf).unwrap();
            }
            Err(e) => log::error!("Get error while trying to serialize block: {e:?}")
        }

    }
}
