use std::path::Path;

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
}
