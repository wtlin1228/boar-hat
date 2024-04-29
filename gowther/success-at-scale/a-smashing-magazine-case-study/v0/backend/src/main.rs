use actix_files::NamedFile;
use actix_web::{rt::time::sleep, HttpRequest};
use std::{path::PathBuf, time::Duration};

const FRONTEND_PATH: &'static str = "../frontend";

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let filename = req.match_info().query("filename");
    match filename {
        "index.html" => sleep(Duration::from_millis(500)).await,
        _ => sleep(Duration::from_millis(1000)).await,
    }
    let path = PathBuf::from(FRONTEND_PATH).join(filename);
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
