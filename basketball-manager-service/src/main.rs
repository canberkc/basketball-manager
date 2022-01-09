use actix_web::{App, HttpResponse, HttpServer, Responder, web, get, post};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health_check)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

#[get("/greet")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Basketball Manager!")
}

#[post("/health")]
async fn health_check(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
