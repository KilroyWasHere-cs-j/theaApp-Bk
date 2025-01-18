use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;
use serde_json::json;
use chrono; 

// Vars shared across all routes
// Use Mutex not arc to protect
pub struct TheaState<'a>{
    pub app_name: &'a str,
}

pub enum SeatStats {
    EMPTY,
    TAKEN,
    HOLDING,
    MISSING
}

// index
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Thea theater ticket managment API")
}

// Determine if path is alive
#[get("/isAlive")]
async fn isAlive() -> impl Responder {
    let curT = chrono::offset::Local::now().to_string();
    let resp = json!({
        "isAlive": "yes",
        "time":  curT,
    });
    web::Json(resp)
}

// Route to document
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

/// Converts a Theater Nsme to a TID
#[get("/getTID/{name}")]
async fn getTID(path: web::Path<(String)>) -> impl Responder {
    let tid = 0000;
    let rtn = json!({
        "given": path.into_inner(),
        "TID": tid,
    });
    web::Json(rtn)
}

// Converts a TID to a Theater Name
#[get("/getTheatNam/{TID}")]
async fn getTheatNam(path: web::Path<(String)>) -> impl Responder {
    let theatNam = "Theater Theater";
    let rtn = json!({
        "given": path.into_inner(),
        "TheatNam": theatNam,
    });
    web::Json(rtn)
}

#[get("/checkSeat/{seatID}")]
async fn checkSeat(path: web::Path<(String)>) -> impl Responder {
    let sAval = false;
    let rtn = json!({
        "given": path.into_inner(),
        "aval": false,
    });
    web::Json(rtn)
}

#[get("/checkSeatAll")]
async fn checkSeatAll() -> impl Responder {
    HttpResponse::Ok().body("Not here yet")
}

#[get("/setSeat/{seatID}/{status}")]
async fn setSeat(path: web::Path<(String, String)>) -> impl Responder {
    let (seatID, status) = path.into_inner();
    let mut duck = false;

    if status == "TAKEN" {
        duck = true
    }

    let rtn = json!({
        "givenSeatID": seatID,
        "givenStatus": status,
        "seatID": seatID,
        "nowStatus": "NULL",
        "tick": duck
    });
    web::Json(rtn)
}






























