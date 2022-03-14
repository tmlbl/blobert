use std::path::PathBuf;
use std::fs::File;
use log::debug;

pub struct Store {
    dir: PathBuf
}

impl Store {
    pub fn new(dir: &str) -> Store {
        let dir = PathBuf::from(dir);
        if !dir.exists() {
            std::fs::create_dir_all(&dir).unwrap();
            let mut upload = dir.clone();
            upload.push("upload");
            std::fs::create_dir_all(upload).unwrap();
        }
        Store { dir }
    }

    pub fn get_upload_file(&self, id: &str) -> Result<File, std::io::Error> {
        let mut path = PathBuf::from(&self.dir);
        path.push("upload");
        path.push(id);
        
        if !path.exists() {
            debug!("Creating upload temp file at {}",  path.to_str().unwrap());
            File::create(&path)
        } else {
            File::open(&path)
        }
    }
}
