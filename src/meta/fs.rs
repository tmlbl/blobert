use std::{path::PathBuf, os::unix::fs};

use crate::meta::{Store, Manifest};

pub struct Filesystem {
    data_dir: String
}

impl Filesystem {
    pub fn new(dir: &str) -> Result<Filesystem, std::io::Error> {
        match std::fs::create_dir_all(dir) {
            Ok(_) => Ok(Filesystem {
                data_dir: dir.to_owned()
            }),
            Err(e) => Err(e)
        }
    }

    fn get_manifest_path(&self, namespace: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.data_dir);
        path.push("manifests");
        path.push(namespace);
        std::fs::create_dir_all(&path).unwrap();
        path
    }
}

impl Store for Filesystem {
    fn put_manifest(&self, namespace: &str, tag: &str, m: &Manifest) -> Result<(), std::io::Error> {
        let mut tag_path = self.get_manifest_path(namespace);
        tag_path.push(tag);

        let mut sha_path = self.get_manifest_path(namespace);
        sha_path.push(m.digest());

        match std::fs::write(&sha_path, serde_json::to_vec(m).unwrap()) {
            Err(e) => Err(e),
            Ok(_) => {
                fs::symlink(sha_path, tag_path)
            }
        }
    }

    fn get_manifest(&self, namespace: &str, reference: &str) -> Result<Manifest, std::io::Error> {
        let mut path = self.get_manifest_path(namespace);
        path.push(reference);

        match std::fs::read(path) {
            Ok(data) => Ok(serde_json::from_slice(&data).unwrap()),
            Err(e) => Err(e)
        }
    }

    fn list_tags(&self, namespace: &str) -> Vec<String> {
        let dir = std::fs::read_dir(self.get_manifest_path(namespace)).unwrap();
        let mut tags: Vec<String> = Vec::new();
        for (_, entry) in dir.enumerate() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_symlink() {
                tags.push(String::from(entry.file_name().to_str().unwrap()));
            }
        }
        tags
    }
}