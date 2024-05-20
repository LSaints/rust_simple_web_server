mod routes;

use actix_web::{web, App, HttpServer};
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server on: http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
