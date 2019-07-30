mod videos;
mod config;
use actix_web::{middleware::Logger, web,App,HttpResponse,HttpRequest,HttpServer,Responder,http::Method,Result};
use actix_session::{Session, CookieSession};
use std::sync::Mutex;
use log::debug;
use actix_files;
use tera::Tera;
use serde::{Serialize,Deserialize};
mod users;
#[derive(Clone)]
pub struct State{
    pub config_file: config::Config,
    pub video_array: Vec<videos::Video>,
    pub users: users::UserVec,
}
impl State{
    //returns cookie if user is suscessfully authenticated
    pub fn authUser(&self,username:String,password:String)->Result<String,String>{
        if self.users.verifyUser(username.clone(),password){
            return Ok(self.users.getToken(username.clone()).unwrap())
        }
        return Err("invalid credentials".to_string())
    }
	pub fn isAuth(&self,token:String)->bool{
		return self.users.verifyToken(token);
	}
    pub fn addUser(&mut self,username:String,password:String,user_token:String)->Result<String,String>{
        if self.users.verifyToken(user_token){
            self.users.addUser(username,password);
			self.write();
            return Ok("Sucess".to_string());
        }
        return Err("not authorized".to_string());
    }
	pub fn getVideos(&self,user_token:String)->Vec<videos::Video_html>{
		let mut out:Vec<videos::Video_html>=Vec::new();
		for vid in self.video_array.clone(){
			out.push(vid.getVid_html("/videos/".to_string(),"/thumbnails/".to_string()));	
		}
		return out;
	}
	pub fn getVidHtml(&self,user_token:String,video_name:String)->Result<videos::Video_html,String>{
		if self.users.verifyToken(user_token){
			for vid in self.video_array.clone(){
				if vid.name==video_name{
					return Ok(vid.getVid_html("/videos/".to_string(),"/thumbnails/".to_string()));
				}
			}
			return Err("not found".to_string());
		}else{
			return Err("not authorized".to_string())
		}
	}
	pub fn getVidDir(&self)->String{
		return self.config_file.videos.video_path.clone();
	}
        pub fn getThumbDir(&self)->String{
            return self.config_file.videos.thumbnails.clone();
        }
    //adds the root user
    pub fn addRoot(&mut self,username:String,password: String){
        assert!(self.users.isEmpty());
        self.users.addUser(username,password);
		self.write();

    }
    pub fn printUsers(&self){
        println!("{}",self.users.printUsers());    
    }
	fn write(&mut self){
		let temp_user = self.users.retConfUsers();
		let mut users_write:Vec<config::User>=Vec::new();
		for user in temp_user{
			users_write.push(config::User{
				username: user.username,
				passwd: user.password
			});
		}
		self.config_file.users=users_write;
		config::write_conf(self.config_file.clone());
	}
}
lazy_static!{
	pub static ref TERA: Tera = {
		let tera = compile_templates!("templates/**/*");
		tera
	};
}
fn init_state()->std::result::Result<State,String>{
    let temp_cfg=config::load_config();
    if temp_cfg.is_ok(){
        let cfg = temp_cfg.ok().unwrap();
        let vid_dir=cfg.videos.video_path.clone();

        let mut out=State{
            config_file: cfg,
            video_array: videos::get_videos(vid_dir,"thumbnails".to_string()),
            users: users::new(),
        };

        return Ok(out);
    }
    println!("error: {}",temp_cfg.clone().err().unwrap());
    return Err(temp_cfg.err().unwrap());

}
//returns an empty state
fn empty_state()->State{
    return State{
        config_file: config::empty(),
        video_array: [].to_vec(),
        users: users::new()
    }
}
//This runs the webserver used to setup hermit
//
pub fn setup_webserver()->State{
    //todo
    return empty_state();
    let mut state:State =empty_state();
    /*
    HttpServer::new(move || {

    }).bind("127.0.0.1:8088").unwrap().run().unwrap();
    */
}
pub fn run_webserver(state_in:&mut State){
	let video_dir = state_in.getVidDir();
    let thumb_dir= state_in.getThumbDir();
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
			.route("/api/videos",web::get().to(get_videos))
			.route("/api/add_user",web::post().to(add_user))
			.route("/vid_html/{name}",web::get().to(vid_html))
            .route("/", web::get().to(index))
            .route("/login",web::get().to(login_html))

            .service(actix_files::Files::new("/static","./static/"))
        	.service(actix_files::Files::new("/videos",video_dir.clone()))
            .service(actix_files::Files::new("/thumbnails",thumb_dir.clone()))
			
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
pub fn init(){
    let mut state_struct = init_state();
    if state_struct.is_ok(){
        let mut state = state_struct.ok().unwrap();
        state.addRoot("root".to_string(),"password".to_string());
        run_webserver(&mut state);
    }
    println!("state not ok!");
    let mut state = setup_webserver();
    state.write();
    run_webserver(&mut state);
}
#[derive(Deserialize)]
struct UserReq{
    username: String,
    password: String,
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
    let mut state_data = data.lock().unwrap();
    state_data.printUsers();
    let res = state_data.addUser(username.clone(),password.clone(),token);
    if res.is_ok(){
        println!("Added Username: {} Password: {}",username,password);
        return Ok("sucess".to_string());
    }
    return Ok("failed".to_string());
}
fn get_videos(data:web::Data<Mutex<State>>,session:Session)->impl Responder{
	let token = session.get("token").unwrap().unwrap();
	let state_data = data.lock().unwrap();
	let videos=state_data.getVideos(token);
	let out=serde_json::to_string(&videos).unwrap();
	return HttpResponse::Ok().body(out);	
}
#[derive(Serialize)]
struct Index{
	videos: Vec<videos::Video_html>
}
pub fn index(data:web::Data<Mutex<State>>, session:Session)->impl Responder{
	let token:String = session.get("token").unwrap().unwrap();
	let mut state_data = data.lock().unwrap();
	let auth = state_data.isAuth(token.clone());
	if(auth){
		let index_data=Index{
			videos:state_data.getVideos(token)
		};
		let mut data = TERA.render("home.jinja2",&index_data);
		if(data.is_ok()){
			return HttpResponse::Ok().body(data.unwrap());
		}else{
			println!("data not rendered");
		}
	}else{
		println!("not authorized");
        return HttpResponse::TemporaryRedirect().header("Location", "/login").finish();
	}
	let mut out:String = "".to_string();

    HttpResponse::Ok().body(out)
        
}
#[derive(Serialize)]
struct login_struct{

}
pub fn login_html(data:web::Data<Mutex<State>>, session:Session) -> impl Responder{
    println!("ran redirect");
    let mut state_data = data.lock().unwrap();
    let mut html = TERA.render("login.jinja2",&login_struct{});
    if html.is_ok(){
        return HttpResponse::Ok().body(html.unwrap());
    }
    else{
        println!("failed to render body");
        return HttpResponse::InternalServerError().body("");
    }
}
pub fn vid_html(data:web::Data<Mutex<State>>,session:Session,path: web::Path<(String,)>)->HttpResponse{

	let token:String = session.get("token").unwrap().unwrap();
	let vid_name:String = path.0.clone();
	let mut state_data = data.lock().unwrap();
	let vid_res = state_data.getVidHtml(token,vid_name.clone());
	if vid_res.is_ok(){

		let vid:videos::Video_html = vid_res.unwrap();
		let mut data=TERA.render("video.jinja2",&vid);
		if data.is_ok(){
			return HttpResponse::Ok().body(data.unwrap());
		}else{
			println!("did not process template correctly");
		}
	}
	else{
		println!("did not get video");
	}
	//then use videos.jinja2 to create the data and return it
		
    HttpResponse::Ok().body(vid_name)
}
