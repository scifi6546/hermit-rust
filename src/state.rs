mod videos;
mod config;
use actix_web::{web,App,HttpResponse,HttpServer,Responder};
mod webserver;
mod users;
#[derive(Clone)]
pub struct State{
    config_file: config::Config,
    video_array: Vec<videos::Video>
}
impl State{
    pub fn getVidDir(&self)->String{
        return "Test".to_string();
    }
}
fn init_state()->State{
    let temp_cfg=config::load_config();

    let vid_dir=temp_cfg.videos.video_path.clone();
    let mut out=State{
        config_file: temp_cfg,
        video_array:videos::get_videos(vid_dir)
    };

    return out;

}
pub fn setup_webserver(state_in:State){
    HttpServer::new(move || {
        App::new().data(state_in.clone())
            .route("/", web::get().to(webserver::index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
pub fn init(){
    let state_struct = init_state();
    let video_arr = videos::get_videos(state_struct.config_file.videos.video_path.clone()); 
    let users = users::add_user([].to_vec(),"user".to_string(),"password".to_string());
    assert!(users::verify_user(users,"user".to_string(),"password".to_string()));
    setup_webserver(state_struct);
}
