use actix_web::{error::PayloadError, web, HttpRequest, HttpResponse, Responder};
// use oci_distribution::manifest::OciManifest;
use futures::StreamExt;
use log::{debug, error};
use serde::Serialize;

use crate::meta::IMAGE_MANIFEST_MEDIA_TYPE;
// use crate::util::*;
use crate::Blobert;

#[derive(Serialize)]
struct PutManifestResponse {
    name: String,
    tags: Vec<String>,
}

pub async fn get_manifest(req: HttpRequest) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let namespace = req.match_info().get("namespace").unwrap();
    let reference = req.match_info().get("reference").unwrap();

    match blobert.meta_store.get_manifest(namespace, reference) {
        Ok(manifest) => {
            let payload = serde_json::to_vec(&manifest).unwrap();
            HttpResponse::Ok()
                .append_header(("Content-Type", IMAGE_MANIFEST_MEDIA_TYPE))
                .append_header(("Content-Length", format!("{}", payload.len())))
                .append_header(("Docker-Content-Digest", manifest.digest()))
                .body(payload)
        }
        Err(e) => {
            error!(
                "Error retrieving manifest {}/{}: {}",
                namespace, reference, e
            );
            e.respond()
        }
    }
}

pub async fn put_manifest(req: HttpRequest, mut payload: web::Payload) -> impl Responder {
    debug!("put_manifest");
    let blobert: &Blobert = req.app_data().unwrap();
    let namespace = req.match_info().get("namespace").unwrap();
    let reference = req.match_info().get("reference").unwrap();

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(bytes) => body.extend_from_slice(&bytes),
            Err(e) => {
                error!("payload next error: {:?}", e);
                return Err(e);
            }
        }
    }

    let manifest = match serde_json::from_slice(&body) {
        Err(e) => {
            error!("Error decoding manifest: {}", e);
            return Ok(HttpResponse::BadRequest().finish());
        }
        Ok(man) => man,
    };
    // let data = serde_json::to_vec(&manifest).unwrap();
    // debug!("manifest: {:?}", manifest);
    // debug!("mainfest: {}", String::from_utf8(body.to_vec()).unwrap());
    // debug!("mainfest: {}", serde_json::to_string(&manifest).unwrap());

    match blobert
        .meta_store
        .put_manifest(namespace, reference, &manifest)
    {
        Ok(_) => {
            let tags = blobert.meta_store.list_tags(namespace);
            let response = PutManifestResponse {
                name: reference.to_string(),
                tags,
            };
            let location = format!(
                "{}/v2/{}/manifests/{}",
                blobert.get_server_url(),
                namespace,
                reference
            );
            let man_bytes = serde_json::to_vec(&response).unwrap();
            let digest = manifest.digest();
            debug!("Manifest {}/{} hash: {}", namespace, reference, digest);

            Ok(HttpResponse::Created()
                .append_header(("Content-Type", "application/json"))
                .append_header(("Location", location))
                .append_header(("Docker-Content-Digest", digest))
                .body(man_bytes))
        }
        Err(e) => {
            error!("Error storing manifest file: {}", e);
            Err(PayloadError::EncodingCorrupted)
        }
    }
}
