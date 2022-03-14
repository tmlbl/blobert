use std::fs::File;
use std::io::Write;
use uuid::Uuid;

// Blob store backed by local filesystem
pub struct BlobFile {
    path: String,
    f: File
}

impl BlobFile {
    pub fn new() -> Result<BlobFile, std::io::Error> {
        let id = Uuid::new_v4();
        let path = format!("/tmp/{}", id);
        match File::create(&path) {
            Ok(f) => Ok(BlobFile { path, f }),
            Err(e) => Err(e)
        }
    } 

    pub fn write_chunk(&mut self, chunk: bytes::Bytes) {
        self.f.write(chunk.as_ref()).unwrap();
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

struct Store {
    dir: String
}

impl Store {
    // Create a unique temp file to store upload data, which will later be
    // renamed
    // fn get_temp_file() -> BlobFile {

    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_temp_file() {
        let blob = BlobFile::new().unwrap();
        assert!(std::path::Path::new(&blob.path).exists());
    }
}
