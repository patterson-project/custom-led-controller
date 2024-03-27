use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod config 

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

#[get("/temperature")]
async fn strip_set_temperature(req_body: String) -> impl Responder {
    HttpResponse::Ok(req_body: String).body("Setting temperature")
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