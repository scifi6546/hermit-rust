#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

pub fn setup_webserver(){

    let mut server = Nickel::new();
    server.get("/",middleware!{returnStr()});
    server.utilize(StaticFilesHandler::new("static/"));
    server.listen("127.0.0.1:8080");
    println!("{}",returnStr());
}
