use actix_web::{web, HttpRequest, HttpResponse, Responder, error::PayloadError};
// use oci_distribution::manifest::OciManifest;
use futures::StreamExt;
use log::{error, debug};
use serde::Serialize;
use sha2::Digest;

use crate::Blobert;

#[derive(Serialize)]
struct PutManifestResponse {
    name: String,
    tags: Vec<String>
}

/// Computes the SHA256 digest of a byte vector
fn sha256_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", sha2::Sha256::digest(bytes))
}

pub async fn put_manifest(req: HttpRequest, mut payload: web::Payload) -> impl Responder {
    let blobert: &Blobert = req.app_data().unwrap();
    let namespace = req.match_info().get("namespace").unwrap();
    let reference = req.match_info().get("reference").unwrap();

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(bytes) => body.extend_from_slice(&bytes),
            Err(e) => return Err(e)
        }
    }

    let manifest = match serde_json::from_slice(&body) {
        Err(e) => {
            error!("Error decoding manifest: {}", e);
            return Ok(HttpResponse::BadRequest().finish())
        },
        Ok(man) => man,
    };

    match blobert.meta_store.put_manifest(namespace, reference, &manifest) {
        Ok(_) => {
            let tags = blobert.meta_store.list_tags(namespace);
            let response = PutManifestResponse {
                name: reference.to_string(),
                tags,
            };
            let location = format!("{}/v2/{}/manifests/{}", 
                blobert.opts.get_server_url(), namespace, reference);
            let man_bytes = serde_json::to_vec(&response).unwrap();
            let digest = sha256_digest(&man_bytes);
            debug!("Manifest {}/{} hash: {}", namespace, reference, digest);

            Ok(HttpResponse::Created()
                .append_header(("Content-Type", "application/json"))
                .append_header(("Location", location))
                .append_header(("Docker-Content-Digest", digest))
                .body(man_bytes))
        },
        Err(e) => {
            error!("Error storing manifest file: {}", e);
            Err(PayloadError::EncodingCorrupted)
        }
    }
}
