use crate::error;

use futures::Stream;
use log::debug;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Store {
    dir: PathBuf,
    buf_size: usize,
}

pub struct BlobStream {
    file: File,
    buf_size: usize,
}

impl BlobStream {
    pub fn from_file(path: &str, buf_size: usize) -> Result<BlobStream, std::io::Error> {
        debug!("Opening blob file {}", path);
        let file = File::open(path)?;
        Ok(BlobStream { file, buf_size })
    }
}

impl Stream for BlobStream {
    type Item = Result<bytes::Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = bytes::BytesMut::with_capacity(self.buf_size);
        buf.resize(self.buf_size, 0);
        let read = self.file.read(&mut buf)?;
        debug!("Read {} bytes", read);
        if read == 0 {
            return Poll::Ready(None);
        }
        buf.truncate(read);
        Poll::Ready(Some(Ok(bytes::Bytes::from(buf))))
    }
}

impl Store {
    pub fn new(dir: &str, buf_size: usize) -> Store {
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
        Store { dir, buf_size }
    }

    pub fn get_upload_path(&self, id: &str) -> PathBuf {
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

    pub fn get_blob(&self, digest: &str) -> Result<BlobStream, error::RegistryError> {
        let path = self.get_blob_path(digest);
        match BlobStream::from_file(path.to_str().unwrap(), self.buf_size) {
            Ok(stream) => Ok(stream),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    Err(error::RegistryError::from(error::BLOB_UNKNOWN))
                }
                _ => Err(error::RegistryError::from(error::BLOB_UNKNOWN)),
            },
        }
    }

    pub fn get_upload_file(&self, id: &str) -> Result<File, std::io::Error> {
        let path = self.get_upload_path(id);
        if !path.exists() {
            debug!("Creating upload temp file at {}", path.to_str().unwrap());
            File::create(&path)
        } else {
            File::options().append(true).open(&path)
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
