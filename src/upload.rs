use actix_web::web;
use actix_web::{Responder, HttpRequest, HttpResponse};
use futures::StreamExt;
use uuid::Uuid;
use log::error;
use std::io::Write;

use crate::Blobert;

pub async fn start_blob_upload(req: HttpRequest) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let id = Uuid::new_v4();
    let namespace = req.match_info().get("namespace").unwrap();
    let location = format!("{}/v2/{}/blobs/upload/{}", 
            blobert.opts.get_server_url(), namespace, id);

    HttpResponse::Accepted()
        .append_header(("Location", location))
        .append_header(("Docker-Upload-UUID", id.to_string()))
        .finish()
}

pub async fn patch_blob_data(req: HttpRequest, mut payload: web::Payload) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let namespace = req.match_info().get("namespace").unwrap();
    let id = req.match_info().get("id").unwrap();

    let mut blobfile = match blobert.store.get_upload_file(id) {
        Ok(f) => f,
        Err(e) => {
            error!("Error getting upload file: {}", e);
            return HttpResponse::InternalServerError().finish()
        },
    };

    let mut written: usize = 0;

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                match blobfile.write(&chunk) {
                    Ok(size) => written += size,
                    Err(e) => {
                        error!("Error writing upload file: {}", e);
                        return HttpResponse::InternalServerError().finish()
                    }
                }
            },
            Err(e) => {
                error!("Error getting chunk: {}", e);
                return HttpResponse::InternalServerError().finish()
            }
        }
    }
    let location = format!("{}/v2/{}/blobs/upload/{}", 
            blobert.opts.get_server_url(), namespace, id);
    HttpResponse::Accepted()
        .append_header(("Location", location))
        .append_header(("Docker-Upload-UUID", id.to_string()))
        .append_header(("Content-Length", "0"))
        .append_header(("Range", format!("0-{}", written)))
        .finish()
}
