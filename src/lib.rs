use actix_web::{HttpResponse, Responder};

pub mod blob;
pub mod error;
pub mod manifests;
pub mod meta;
pub mod tls;
pub mod upload;
pub mod util;

pub struct Blobert {
    pub server_url: String,
    pub meta_store: Box<dyn meta::Store>,
    pub blob_store: blob::Store,
}

impl Blobert {
    pub fn new(server_url: String) -> Blobert {
        let meta_store = meta::fs::Filesystem::new("/var/local/register").unwrap();
        Blobert {
            server_url,
            meta_store: Box::new(meta_store),
            blob_store: blob::Store::new("/var/local/register", 10 * 1024 * 1024),
        }
    }

    pub fn get_server_url(&self) -> String {
        self.server_url.clone()
    }

    pub async fn v2() -> impl Responder {
        HttpResponse::Ok()
    }
}
