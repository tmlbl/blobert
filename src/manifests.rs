use actix_web::{web, HttpRequest, HttpResponse, Responder};
use oci_distribution::manifest::OciManifest;
use futures::StreamExt;

struct PutManifestResponse {
    name: String,
    tags: Vec<String>
}

pub async fn put_manifest(req: HttpRequest, mut payload: web::Payload) -> impl Responder {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(bytes) => body.extend_from_slice(&bytes),
            Err(e) => return Err(e)
        }
    }

    println!("Payload: {}", std::str::from_utf8(&body).unwrap());

    Ok(HttpResponse::Ok())
}
