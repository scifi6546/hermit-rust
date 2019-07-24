mod config;
#[macro_use] extern crate nickel;
mod webserver;
mod videos;
fn main(){
    config::load_config();
    videos::get_videos("videos".to_string());
    webserver::setup_webserver();
}
