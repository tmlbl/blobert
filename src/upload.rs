use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::StreamExt;
use log::{debug, error};
use regex::Regex;
use serde::Deserialize;
use std::io::Write;
use uuid::Uuid;

use crate::meta;
use crate::Blobert;

pub async fn get_blob(req: HttpRequest) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let id = req.match_info().get("id").unwrap();

    let stream = blobert.blob_store.get_blob(id).unwrap();

    HttpResponse::Ok()
        .append_header(("Content-Type", meta::IMAGE_LAYER_MEDIA_TYPE))
        .append_header(("Docker-Content-Digest", id.clone()))
        .streaming(stream)
}

pub async fn start_blob_upload(req: HttpRequest) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let id = Uuid::new_v4();
    let namespace = req.match_info().get("namespace").unwrap();
    let location = format!(
        "{}/v2/{}/blobs/upload/{}",
        blobert.opts.get_server_url(),
        namespace,
        id
    );
    HttpResponse::Accepted()
        .append_header(("Location", location))
        .append_header(("Docker-Upload-UUID", id.to_string()))
        .finish()
}

pub async fn patch_blob_data(
    req: HttpRequest,
    mut payload: web::Payload,
    regex: web::Data<Regex>,
) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let namespace = req.match_info().get("namespace").unwrap();
    let id = req.match_info().get("id").unwrap();

    let mut blobfile = match blobert.blob_store.get_upload_file(id) {
        Ok(f) => f,
        Err(e) => {
            error!("Error getting upload file: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    // let content_length = req.headers().get("content-length");
    let mut written: usize = 0;

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => match blobfile.write(&chunk) {
                Ok(size) => written += size,
                Err(e) => {
                    error!("Error writing upload file: {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            },
            Err(e) => {
                error!("Error getting chunk: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    let location = format!(
        "{}/v2/{}/blobs/upload/{}",
        blobert.opts.get_server_url(),
        namespace,
        id
    );
    HttpResponse::Accepted()
        .append_header(("Location", location))
        .append_header(("Docker-Upload-UUID", id.to_string()))
        .append_header(("Content-Length", "0"))
        .append_header(("Range", format!("0-{}", written)))
        .finish()
}

#[derive(Deserialize)]
pub struct PutDigest {
    digest: String,
}

pub async fn put_blob_upload_complete(
    req: HttpRequest,
    info: web::Query<PutDigest>,
) -> impl Responder {
    debug!("put_blob_upload_complete {:?}", req);
    let blobert: &Blobert = req.app_data().unwrap();
    let id = req.match_info().get("id").unwrap();
    let namespace = req.match_info().get("namespace").unwrap();

    // /v2/<name>/blobs/<digest>
    match blobert.blob_store.commit(id, &info.digest) {
        Ok(_) => {
            let location = format!("/v2/{}/blobs/upload/{}", namespace, info.digest);
            HttpResponse::Created()
                .append_header(("Location", location))
                .append_header(("Content-Length", "0"))
                .append_header(("Docker-Content-Digest", info.digest.clone()))
                .finish()
        }
        Err(e) => {
            error!("Error getting chunk: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn blob_exists(req: HttpRequest) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let digest = req.match_info().get("digest").unwrap();
    match blobert.blob_store.blob_exists(digest) {
        true => HttpResponse::Ok()
            .append_header(("Docker-Content-Digest", digest))
            .finish(),
        false => HttpResponse::NotFound().finish(),
    }
}
