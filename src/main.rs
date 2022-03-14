// use oci_distribution::manifest;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use futures::StreamExt;

mod store;

const bind_addr: &str = "127.0.0.1";
const port: i32 = 8080;

fn get_bind_addr() -> String {
    format!("{bind_addr}:{port}")
}

async fn create_blob_upload(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().get("name").unwrap();
    println!("Namespace: {}", name);
    HttpResponse::Accepted()
        .append_header(("Location", format!("http://{bind_addr}:{port}/upload")))
        .finish()
}

async fn handle_upload(mut payload: web::Payload) -> impl Responder {
    let mut blob = match store::BlobFile::new() {
        Ok(blob) => blob,
        Err(e) => {
            return HttpResponse::InternalServerError()
        },
    };
    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(chunk) => {
                println!("Writing chunk to {}", blob.get_path());
                blob.write_chunk(chunk);
            },
            Err(e) => {
                return HttpResponse::InternalServerError()
            }
        }
    }
    HttpResponse::Accepted()
}

async fn v2() -> impl Responder {
    HttpResponse::Ok().body("true")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let blob = store::BlobFile::new();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/v2/", web::get().to(v2))
            .route("/v2/{name}/blobs/uploads/", web::post().to(create_blob_upload))
            .route("/upload", web::patch().to(handle_upload))
    })
    .bind(get_bind_addr())?
    .run()
    .await
}
