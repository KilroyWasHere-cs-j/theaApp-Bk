use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::time::SystemTime;
use crate::api::routes::{TheaState, isAlive, index, routeLS};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let name = "Thea";
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(TheaState {
                app_name: name,
            }))
            .service(index)
            .service(isAlive)
            .service(
                web::scope("/thea")
                .service(index)
                .service(isAlive)
            )
            .service(
                web::scope("/docs")
                .service(routeLS)
            )
            //.route("/hey", web::get().to(man))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
