use actix_web::{web,App,HttpResponse,HttpServer,Responder};
pub fn index()->impl Responder{
    HttpResponse::Ok().body("Hello World!")
}
