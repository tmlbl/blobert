// use oci_distribution::manifest;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use structopt::StructOpt;
use log::error;

mod util;
mod blob;
mod upload;
mod manifests;
mod meta;

#[derive(StructOpt, Clone)]
#[structopt(name = "blobert", about = "Another OCI registry")]
pub struct Options {
    #[structopt(long, default_value = "http")]
    protocol: String,

    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,

    #[structopt(short, long, default_value = "7000")]
    port: usize,

    #[structopt(short = "log", long, default_value = "info")]
    log_level: String,

    #[structopt(short, long, default_value = "10MB")]
    buf_size: String,
}

impl Options {
    pub fn get_bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_server_url(&self) -> String {
        format!("{}://{}", self.protocol, self.get_bind_addr())
    }

    pub fn get_buf_size_bytes(&self) -> usize {
        match byte_unit::Byte::from_str(&self.buf_size) {
            Ok(bytes) => bytes.get_bytes() as usize,
            Err(e) => {
                error!("Invalid spec for buffer size: {}", e);
                std::process::exit(1)
            }
        }
    }
}

pub struct Blobert {
    pub opts: Options,
    pub meta_store: Box<dyn meta::Store>,
    pub blob_store: blob::Store
}

impl Blobert {
    fn new() -> Blobert {
        let meta_store = meta::fs::Filesystem::new("/tmp/data").unwrap();
        let opts = Options::from_args();
        let buf_size = opts.get_buf_size_bytes();
        Blobert {
            opts,
            meta_store: Box::new(meta_store),
            blob_store: blob::Store::new("/tmp/data", buf_size)
        }
    }

    async fn v2() -> impl Responder {
        HttpResponse::Ok().body("true")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = Options::from_args();
    let bind_addr = opts.get_bind_addr();
    env_logger::init_from_env(Env::default().default_filter_or(&opts.log_level));

    HttpServer::new(move || {
        App::new()
            .app_data(Blobert::new())
            .wrap(Logger::new("%r"))
            .route("/v2/", web::get().to(Blobert::v2))
            .route("/v2/{namespace}/blobs/{id}", web::get().to(upload::get_blob))
            .route("/v2/{namespace}/blobs/uploads/", web::post().to(upload::start_blob_upload))
            .route("/v2/{namespace}/blobs/upload/{id}", web::patch().to(upload::patch_blob_data))
            .route("/v2/{namespace}/blobs/upload/{id}", web::put().to(upload::put_blob_upload_complete))
            .route("/v2/{namespace}/blobs/{digest}", web::head().to(upload::blob_exists))
            .route("/v2/{namespace}/manifests/{reference}", web::put().to(manifests::put_manifest))
            .route("/v2/{namespace}/manifests/{reference}", web::head().to(manifests::get_manifest))
            .route("/v2/{namespace}/manifests/{reference}", web::get().to(manifests::get_manifest))
    })
    .bind(bind_addr)?
    .run()
    .await
}
