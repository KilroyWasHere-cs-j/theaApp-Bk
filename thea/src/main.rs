use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::time::SystemTime;
use crate::api::routes::{TheaState, isAlive, index, routeLS};

mod api;

// Main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Build app state
            .app_data(web::Data::new(TheaState {
                app_name: "Thea",
            }))
            // Defining routes for both "paths"
            .service(index)
            .service(isAlive)
            .service(
                web::scope("/thea")
                .service(index)
                .service(isAlive)
            )
            .service(
                web::scope("/docs")
                .service(isAlive)
                .service(routeLS)
            )
            //.route("/hey", web::get().to(man))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
