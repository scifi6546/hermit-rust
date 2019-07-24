use nickel::{Nickel, HttpRouter, StaticFilesHandler};

pub fn setup_webserver(){

    let mut server = Nickel::new();
    server.utilize(StaticFilesHandler::new("static/"));
    server.listen("127.0.0.1:8080");
}
