use actix_web::{error, get, post, web, Error, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
mod strip;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

// Data transfer objects
#[derive(Serialize, Deserialize)]
struct BrightnessDto {
    brightness: i32,
}

#[derive(Serialize, Deserialize)]
struct HsvDto {
    h: f32,
    s: f32,
    v: f32,
}

#[derive(Serialize, Deserialize)]
struct TemperatureDto {
    temperature: i32,
}

// GET requests
#[get("/on")]
async fn strip_on() -> impl Responder {
    strip::strip_on().await;
    HttpResponse::Ok().body("Turning on the LED strip")
}

#[get("/off")]
async fn strip_off() -> impl Responder {
    strip::strip_off().await;
    HttpResponse::Ok().body("Turning off the LED strip")
}

// POST requests
#[post("/brightness")]
async fn strip_set_brightness(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let brightness = serde_json::from_slice::<BrightnessDto>(&body)?;
    Ok(HttpResponse::Ok().body(format!("Setting brightness: {}", brightness.brightness)))
}

// WIP

#[post("/hsv")]
async fn strip_set_hsv(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let hsv = serde_json::from_slice::<HsvDto>(&body)?;
    Ok(HttpResponse::Ok().body(format!("Setting HSV: {}, {}, {}", hsv.h, hsv.s, hsv.v)))
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
    let temperature = serde_json::from_slice::<TemperatureDto>(&body)?;
    Ok(HttpResponse::Ok().body(format!("Setting temperature: {}", temperature.temperature)))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    strip::init();
    HttpServer::new(|| {
        App::new()
            .service(strip_on)
            .service(strip_off)
            .service(strip_set_brightness)
            .service(strip_set_hsv)
            .service(strip_set_temperature)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}