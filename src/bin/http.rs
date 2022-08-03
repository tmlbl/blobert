use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use blobert::{manifests, upload, Blobert};
use log::LevelFilter::{Debug, Info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = "0.0.0.0:8000";
    custom_utils::logger::logger_feature("register", Debug, Debug)
        .module("h2", Info)
        .build();
    let range_regex = web::Data::new(regex::Regex::new("^([0-9]+)-([0-9]+)$").unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(Blobert::new("https://www.myhost.cn".to_string()))
            .app_data(range_regex.clone())
            .wrap(Logger::new("%r"))
            .route("/v2/", web::get().to(Blobert::v2))
            .route(
                "/v2/{namespace}/blobs/{id}",
                web::get().to(upload::get_blob),
            )
            .route(
                // 获取上传的location（uuid）
                "/v2/{namespace}/blobs/uploads/",
                web::post().to(upload::start_blob_upload),
            )
            .route(
                "/v2/{namespace}/blobs/upload/{id}",
                web::patch().to(upload::patch_blob_data),
            )
            .route(
                // 整体上传
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
    // .bind_rustls(bind_addr, load_rustls_config())?
    .bind(bind_addr)?
    .run()
    .await
}
