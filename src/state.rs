mod videos;
mod config;
use actix_web::{web,App,HttpResponse,HttpRequest,HttpServer,Responder,http::Method};
use actix_session::{Session, CookieSession};
use actix_files;
use serde::Deserialize;
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
    pub fn authUsr(&self,username:String,password:String)->Result<String,String>{
        Ok("hello".to_string())
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
        App::new().data(state_in.clone()).wrap(
            CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .secure(false)
            )
            .route("/api/login",web::post().to(login))
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/static","./static/"))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
pub fn init(){
    let state_struct = init_state();
   // let video_arr = videos::get_videos(state_struct.config_file.videos.video_path.clone()); 
    let users = users::add_user([].to_vec(),"user".to_string(),"password".to_string());
    assert!(users::verify_user(users,"user".to_string(),"password".to_string()));
    setup_webserver(state_struct);
}
#[derive(Deserialize)]
pub struct UserReq{
    pub username: String,
    pub password: String
}
pub fn login(data:web::Data<State>, session:Session, req: HttpRequest,user_req: web::Json<UserReq>)->impl Responder{
    println!("processed login request");
    if req.method()==Method::POST{
        println!("got");
        println!("data: username: {} password: {}",user_req.username,user_req.password);

    }
    else{
        println!("method not found: {}",req.method().as_str());
        return HttpResponse::BadRequest().body("bad user!");
    }
    HttpResponse::Ok().body("Hello World!")
}
pub fn index(data:web::Data<State>, session:Session)->impl Responder{
    let res= data.authUsr("foo".to_string(),"bar".to_string());
    if(res.is_ok()){
        let token:String = res.unwrap();
        session.set("token",token.clone());
        println!("added token: {}",token);
    }
    HttpResponse::Ok().body("Hello World!")
        
}

