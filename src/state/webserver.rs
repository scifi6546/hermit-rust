use actix_web::{web,App,HttpResponse,HttpServer,Responder};
fn index()->impl Responder{
    HttpResponse::Ok().body("Hello World!")
}
pub fn setup_webserver(){
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
    //let mut server = Nickel::new();
    //server.utilize(StaticFilesHandler::new("static/"));
    //server.listen("127.0.0.1:8080");
}
