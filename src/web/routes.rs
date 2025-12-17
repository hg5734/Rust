use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix!")
}

#[get("/echo")]
pub async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Echo route works!")
}
