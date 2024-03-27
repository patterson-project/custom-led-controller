use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize}
mod config;

#[derive(Debug, Serialize, Deserialize)]
struct BrightnessDto {
    brightness: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct HsvDto {
    h: i32,
    s: i32,
    v: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemperatureDto {
    temperature: i32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/on")]
async fn strip_on() -> impl Responder {
    config::main
    HttpResponse::Ok().body("Turning on the LED strip")
}

#[get("/off")]
async fn strip_off() -> impl Responder {
    HttpResponse::Ok().body("Turning off the LED strip")
}

#[post("/temperature")]
async fn strip_set_temperature(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let temp = serde_json::from_slice::<TemperatureDto>(&body)?;
    HttpResponse::Ok().json(temp);
}

#[get("/hsv")]
async fn strip_set_hsv(req_body: String) -> impl Responder {
    HttpResponse::Ok(req_body: String).body("Setting temperature")
}



#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}