use std::{path::PathBuf, os::unix::fs};

use crate::meta::{Store, Manifest};
use crate::error;
use crate::error::RegistryError;

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
    fn put_manifest(&self, namespace: &str, tag: &str, m: &Manifest) -> Result<(), RegistryError> {
        let mut tag_path = self.get_manifest_path(namespace);
        tag_path.push(tag);

        let mut sha_path = self.get_manifest_path(namespace);
        sha_path.push(m.digest());

        // If we already have the manifest at this SHA, skip writing
        if !sha_path.exists() {
            match std::fs::write(&sha_path, serde_json::to_vec(m).unwrap()) {
                Err(e) => {
                    return Err(RegistryError::from_err(error::UNKNOWN_ERROR, Box::new(e)))
                },
                _ => ()
            }
        }
        // Update the symlink
        if tag_path.exists() {
            std::fs::remove_file(&tag_path).unwrap()
        }
        match fs::symlink(sha_path, tag_path) {
            Err(e) => Err(RegistryError::from_err(error::UNKNOWN_ERROR, Box::new(e))),
            _ => Ok(())
        }
    }

    fn get_manifest(&self, namespace: &str, reference: &str) -> Result<Manifest, RegistryError> {
        let mut path = self.get_manifest_path(namespace);
        path.push(reference);

        match std::fs::read(path) {
            Ok(data) => Ok(serde_json::from_slice(&data).unwrap()),
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound =>
                        Err(RegistryError::from_err(error::MANIFEST_UNKNOWN, Box::new(e))),
                    _ => Err(RegistryError::from_err(error::UNKNOWN_ERROR, Box::new(e)))
                }
            }
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
        tags.sort();
        tags
    }
}
