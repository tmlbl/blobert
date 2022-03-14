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

            let mut blobs = dir.clone();
            blobs.push("blobs");
            std::fs::create_dir_all(blobs).unwrap();
        }
        Store { dir }
    }

    fn get_upload_path(&self, id: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.dir);
        path.push("upload");
        path.push(id);
        path
    }

    fn get_blob_path(&self, digest: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.dir);
        path.push("blobs");
        path.push(digest);
        path
    }

    pub fn get_upload_file(&self, id: &str) -> Result<File, std::io::Error> {
        let path = self.get_upload_path(id);
        if !path.exists() {
            debug!("Creating upload temp file at {}",  path.to_str().unwrap());
            File::create(&path)
        } else {
            File::open(&path)
        }
    }

    pub fn commit(&self, id: &str, digest: &str) -> Result<(), std::io::Error> {
        let src = self.get_upload_path(id);
        let dest = self.get_blob_path(digest);
        debug!("Moving {} to {}", src.to_str().unwrap(), dest.to_str().unwrap());
        std::fs::rename(src, dest)
    }

    pub fn blob_exists(&self, digest: &str) -> bool {
        self.get_blob_path(digest).exists()
    }
}
