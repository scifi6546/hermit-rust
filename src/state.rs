mod videos;
mod config;
use actix_web::{middleware::Logger, web,App,HttpResponse,HttpRequest,HttpServer,Responder,http::Method,Result};
use actix_session::{Session, CookieSession};
use std::sync::Mutex;
use log::debug;
use actix_files;
use serde::Deserialize;
mod users;
#[derive(Clone)]
pub struct State{
    config_file: config::Config,
    video_array: Vec<videos::Video>,
    pub users: users::UserVec,

}
impl State{
    pub fn getVidDir(&self)->String{
        return "Test".to_string();
    }
    //returns cookie if user is suscessfully authenticated
    pub fn authUser(&self,username:String,password:String)->Result<String,String>{
        if self.users.verifyUser(username.clone(),password){
            return Ok(self.users.getToken(username.clone()).unwrap())
        }
        return Err("invalid credentials".to_string())
    }
    pub fn addUser(&mut self,username:String,password:String,user_token:String)->Result<String,String>{
        if self.users.verifyToken(user_token){
            self.users.addUser(username,password);
            return Ok("Sucess".to_string());
        }
        return Err("not authorized".to_string());
    }
    //adds the root user
    pub fn addRoot(&mut self,username:String,password: String){
        assert!(self.users.isEmpty());
        self.users.addUser(username,password);

    }
    pub fn printUsers(&self){
        println!("{}",self.users.printUsers());    
    }
}
fn init_state()->State{
    let temp_cfg=config::load_config();

    let vid_dir=temp_cfg.videos.video_path.clone();
    let mut out=State{
        config_file: temp_cfg,
        video_array: videos::get_videos(vid_dir),
        users: users::new(),
    };

    return out;

}
pub fn setup_webserver(state_in:&mut State){
    let temp_state = Mutex::new(state_in.clone());
    let mut shared_state = web::Data::new(temp_state);
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init();
    HttpServer::new(move || {
        App::new().wrap(
            CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .secure(false)
            ).wrap( Logger::default())
			.register_data(shared_state.clone())
            .route("/api/login",web::post().to(login))
			.route("/api/add_user",web::post().to(add_user))
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/static","./static/"))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
pub fn init(){
    let mut state_struct = init_state();
    state_struct.addRoot("root".to_string(),"password".to_string());
    setup_webserver(&mut state_struct);
}
#[derive(Deserialize)]
struct UserReq{
    username: String,
    password: String,
}
#[derive(Deserialize)]
struct Test{
    foo:String
}
fn login(info: web::Json<UserReq>, data:web::Data<Mutex<State>>,session:Session)-> Result<String>{
    println!("Processed Username: {} Password: {}",info.username,info.password);
	let mut state_data=data.lock().unwrap();
    let auth=state_data.authUser(info.username.clone(),info.password.clone());
    if auth.is_ok(){
        println!("Authenticated Username: {} Password: {}",info.username,info.password);
        session.set("token",auth.unwrap());
        return Ok("logged in sucessfully".to_string());
    }
    else{
        println!("Denied Username: {} Password: {}",info.username,info.password);
        return Ok("Login Failed".to_string());

    }
    return Ok("hello".to_string());
}
fn add_user(info:web::Json<UserReq>,data:web::Data<Mutex<State>>,session:Session)->Result<String>{
    let token = session.get("token").unwrap().unwrap();
    let username = info.username.clone();
    let password = info.password.clone();
    //let use_data = data.get_ref().unwrap();
    //use_data.wtf();
    let mut state_data = data.lock().unwrap();
    state_data.printUsers();
    let res = state_data.addUser(username.clone(),password.clone(),token);
    if res.is_ok(){
        println!("Added Username: {} Password: {}",username,password);
        return Ok("sucess".to_string());
    }
    return Ok("failed".to_string());
}
pub fn index(data:web::Data<Mutex<State>>, session:Session)->impl Responder{
    HttpResponse::Ok().body("Hello World!")
        
}

