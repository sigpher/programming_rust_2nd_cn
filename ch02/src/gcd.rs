use actix_web::{web, App, HttpServer};
use ch02::handler::{get_index, post_gcd};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
