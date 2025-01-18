use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;
use serde_json::json;
use chrono; 

pub struct TheaState<'a>{
    pub app_name: &'a str,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Thea theater ticket managment API")
}

#[get("/isAlive")]
async fn isAlive() -> impl Responder {
    let curT = chrono::offset::Local::now().to_string();
    let resp = json!({
        "isAlive": "yes",
        "time":  curT,
    });
    web::Json(resp)
}

#[get("/routesLS")]
async fn routeLS() -> impl Responder {
    let ls = r#"
    This is a list of routes supported in this API.
    The API for now has two scopes "thea" and "docs" thea is the actual API docs is just for documentation and anything else.

    /thea
        / - index
        /isAlive - if this doesn't return the server is not alive

    /docs
        /routesLS - this page...

    Kilroy Was Here
    "#;
    HttpResponse::Ok().body(ls)
}

