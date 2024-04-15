use actix_files::NamedFile;
use actix_web::{get, http::header::ContentType, rt::time::sleep, web, HttpRequest, HttpResponse};
use async_stream::stream;
use std::{convert::Infallible, path::PathBuf, time::Duration};

const FRONTEND_PATH: &'static str = "../frontend";

async fn files(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let filename = req.match_info().query("filename");
    match filename {
        "css/index.css" => sleep(Duration::from_millis(1000)).await,
        "js/Common.js" => sleep(Duration::from_millis(1000)).await,
        "js/Consumer.js" => sleep(Duration::from_millis(2000)).await,
        "js/PostFeed.js" => sleep(Duration::from_millis(3000)).await,
        _ => sleep(Duration::from_millis(1000)).await,
    }
    let path = PathBuf::from(FRONTEND_PATH).join(filename);
    Ok(NamedFile::open(path)?)
}

#[get("/index.html")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .streaming(stream! {
            yield Ok::<_, Infallible>(web::Bytes::from("
                <!DOCTYPE html>
                <html lang=\"en\">
                <head>
                    <meta charset=\"UTF-8\" />
                    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
                    <title>Document</title>
                
                    <link rel=\"stylesheet\" href=\"/css/index.css\" />
                
                    <!-- https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel/preload#cors-enabled_fetches -->
                    <link
                        rel=\"preload\"
                        href=\"/api/posts\"
                        as=\"fetch\"
                        type=\"application/json\"
                        crossorigin
                    />
                    <!-- https://web.dev/articles/modulepreload -->
                    <link rel=\"modulepreload\" href=\"/js/PostFeed.js\" />
                
                    <!-- critical path scripts to load the initial page -->
                    <script src=\"/js/Common.js\" type=\"module\"></script>
                    <script src=\"/js/Consumer.js\" type=\"module\"></script>
                </head>            
            "));

            sleep(Duration::from_millis(500)).await;

            yield Ok::<_, Infallible>(web::Bytes::from("
                <body>
                    <div id=\"resource-loading-state\">
                    <div id=\"Common-js\">Common.js <span>❓</span></div>
                    <div id=\"Consumer-js\">Consumer.js <span>❓</span></div>
                    <div id=\"PostFeed-js\">PostFeed.js <span>❓</span></div>
                    <div id=\"get-posts-api\">GET /api/posts <span>❓</span></div>
                    </div>
                    <header>Header</header>
                    <main></main>
                    <footer>Footer</footer>
                </body>
                </html>
            "));
        })
}

#[get("/api/posts")]
async fn posts() -> actix_web::Result<NamedFile> {
    sleep(Duration::from_millis(2000)).await;
    let path: PathBuf = PathBuf::from(FRONTEND_PATH).join("posts.json");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(posts)
            .service(index)
            .route("/{filename:.*}", web::get().to(files))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
