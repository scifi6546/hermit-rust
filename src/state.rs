mod videos;
mod config;
mod webserver;
pub fn init(){
    let config_struct = config::load_config();

    let video_arr = videos::get_videos(config_struct.videos.video_path); 
    webserver::setup_webserver();
}
