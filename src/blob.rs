use log::debug;
use futures::Stream;

use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Poll, Context};

pub struct Store {
    dir: PathBuf,
}

pub struct BlobStream {
    file: File,
}

impl BlobStream {
    pub fn from_file(path: &str) -> Result<BlobStream, std::io::Error> {
        debug!("Opening blob file {}", path);
        let file = File::open(path)?;
        Ok(BlobStream{ file })
    }
}

impl Stream for BlobStream {
    type Item = Result<bytes::Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = bytes::BytesMut::with_capacity(4096 * 10);
        buf.resize(4096 * 10, 0);
        let read = self.file.read(&mut buf)?;
        debug!("Read {} bytes", read);
        if read == 0 {
            return Poll::Ready(None)
        }
        Poll::Ready(Some(Ok(bytes::Bytes::from(buf))))
    }
}

impl Store {
    pub fn new(dir: &str) -> Store {
        let dir = PathBuf::from(dir);
        debug!("Creating data directory: {}", dir.to_str().unwrap());
        std::fs::create_dir_all(&dir).unwrap();

        let mut upload = dir.clone();
        upload.push("upload");
        std::fs::create_dir_all(upload).unwrap();

        let mut blobs = dir.clone();
        blobs.push("blobs");
        std::fs::create_dir_all(blobs).unwrap();

        let mut manifests = dir.clone();
        manifests.push("manifests");
        std::fs::create_dir_all(manifests).unwrap();
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

    pub fn get_blob(&self, digest: &str) -> Result<BlobStream, std::io::Error> {
        let path = self.get_blob_path(digest);
        BlobStream::from_file(path.to_str().unwrap())
    }

    pub fn get_upload_file(&self, id: &str) -> Result<File, std::io::Error> {
        let path = self.get_upload_path(id);
        if !path.exists() {
            debug!("Creating upload temp file at {}", path.to_str().unwrap());
            File::create(&path)
        } else {
            File::open(&path)
        }
    }

    pub fn commit(&self, id: &str, digest: &str) -> Result<(), std::io::Error> {
        let src = self.get_upload_path(id);
        let dest = self.get_blob_path(digest);
        debug!(
            "Moving {} to {}",
            src.to_str().unwrap(),
            dest.to_str().unwrap()
        );
        std::fs::rename(src, dest)
    }

    pub fn blob_exists(&self, digest: &str) -> bool {
        self.get_blob_path(digest).exists()
    }
}
