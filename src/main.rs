use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
// use oci_distribution::manifest;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::error;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, rsa_private_keys};
use structopt::StructOpt;

mod blob;
mod error;
mod manifests;
mod meta;
mod upload;
mod util;

#[derive(StructOpt, Clone)]
#[structopt(name = "blobert", about = "Another OCI registry")]
pub struct Options {
    #[structopt(long, default_value = "https")]
    protocol: String,

    #[structopt(short, long, default_value = "www.duduwuli.cn")]
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
    pub blob_store: blob::Store,
}

impl Blobert {
    fn new() -> Blobert {
        let meta_store = meta::fs::Filesystem::new("/tmp/data").unwrap();
        let opts = Options::from_args();
        let buf_size = opts.get_buf_size_bytes();
        Blobert {
            opts,
            meta_store: Box::new(meta_store),
            blob_store: blob::Store::new("/tmp/data", buf_size),
        }
    }

    async fn v2() -> impl Responder {
        HttpResponse::Ok()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = Options::from_args();
    let bind_addr = opts.get_bind_addr();
    env_logger::init_from_env(Env::default().default_filter_or(&opts.log_level));

    let range_regex = web::Data::new(regex::Regex::new("^([0-9]+)-([0-9]+)$").unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(Blobert::new())
            .app_data(range_regex.clone())
            .wrap(Logger::new("%r"))
            .route("/v2/", web::get().to(Blobert::v2))
            .route(
                "/v2/{namespace}/blobs/{id}",
                web::get().to(upload::get_blob),
            )
            .route(
                "/v2/{namespace}/blobs/uploads/",
                web::post().to(upload::start_blob_upload),
            )
            .route(
                "/v2/{namespace}/blobs/upload/{id}",
                web::patch().to(upload::patch_blob_data),
            )
            .route(
                "/v2/{namespace}/blobs/upload/{id}",
                web::put().to(upload::put_blob_upload_complete),
            )
            .route(
                "/v2/{namespace}/blobs/{digest}",
                web::head().to(upload::blob_exists),
            )
            .route(
                "/v2/{namespace}/manifests/{reference}",
                web::put().to(manifests::put_manifest),
            )
            .route(
                "/v2/{namespace}/manifests/{reference}",
                web::head().to(manifests::get_manifest),
            )
            .route(
                "/v2/{namespace}/manifests/{reference}",
                web::get().to(manifests::get_manifest),
            )
    })
    .bind_rustls(bind_addr, load_rustls_config())?
    // .bind(bind_addr)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("www.duduwuli.cn_bundle.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("www.duduwuli.cn.key").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = rsa_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
