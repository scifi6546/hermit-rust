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
    pub is_setup:bool,
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
            return self._addUser(username,password);
        }
        return Err("not authorized".to_string());
    }
    fn _addUser(&mut self, username:String,password:String)->Result<String,String>{
        self.users.addUser(username,password);
        self.write();
        return Ok("sucess".to_string());
    }
	pub fn getVideos(&self,user_token:String)->Vec<videos::Video_html>{
		let mut out:Vec<videos::Video_html>=Vec::new();
		for vid in self.video_array.clone(){
			out.push(vid.getVid_html("/vid_html/".to_string(),"/thumbnails/".to_string()));	

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
        pub fn isSetup(&self)->bool{
            return self.is_setup;
        }
        pub fn setup(&mut self,video_dir:String, 
                     username:String, 
                     password:String)->Result<String,String>{
            if self.is_setup{
                return Err("already setup".to_string());
            }
            self.reload_server(video_dir);
            self._addUser(username,password);
            self.is_setup=true;
            return Ok("Sucess".to_string());

        }
        pub fn reload_server(&mut self,video_dir:String, 
                     )->Result<String,String>{
            self.config_file.videos.video_path=video_dir.clone();
            self.config_file.videos.thumbnails="thumbnails".to_string();
            self.video_array=videos::get_videos(video_dir.clone(),"thumbnails".to_string());
            return Ok("done".to_string());
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
fn init_state()->State{
    let temp_cfg=config::load_config();
    if temp_cfg.is_ok(){
        let cfg = temp_cfg.ok().unwrap();
        let vid_dir=cfg.videos.video_path.clone();

        let mut out=State{
            config_file: cfg,
            video_array: videos::get_videos(vid_dir,"thumbnails".to_string()),
            users: users::new(),
            is_setup: true,
        };

        return out;
    }
    println!("error: {}",temp_cfg.clone().err().unwrap());
    return empty_state();

}
//returns an empty state
fn empty_state()->State{
    return State{
        config_file: config::empty(),
        video_array: [].to_vec(),
        users: users::new(),
        is_setup: false,
    }
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
            .route("/setup",web::get().to(setup))
            .route("/api/setup",web::post().to(api_setup))

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
    run_webserver(&mut state_struct);
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
        println!("getting token");
        let temp = session.get("token");
        let mut token:String="".to_string();
        if temp.is_ok(){
            let temp_token = temp.ok().unwrap();
            if temp_token.is_some(){
                token=temp_token.unwrap();
            }
        }
        println!("getting state data");
	let mut state_data = data.lock().unwrap();
	let auth = state_data.isAuth(token.clone());
        if !state_data.isSetup(){
            return HttpResponse::TemporaryRedirect().header("Location","/setup").finish();
        }
	if(auth){
		let index_data=Index{
			videos:state_data.getVideos(token)
		};
		let mut out_data = TERA.render("home.jinja2",&index_data);
		if out_data.is_ok(){
			return HttpResponse::Ok().body(out_data.unwrap());
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
pub fn setup(data:web::Data<Mutex<State>>,session:Session)->impl Responder{
        let data = TERA.render("setup.jinja2",&empty_struct{}); 
        if data.is_ok(){
	    return HttpResponse::Ok().body(data.unwrap());
        }
            return HttpResponse::TemporaryRedirect().header("Location","/setup").finish();
}
#[derive(Serialize,Deserialize)]
struct setup_struct{
    video_dir:String,
    thumbnail_dir:String,
    username:String,
    password:String,
}
fn api_setup(info: web::Json<setup_struct>, data:web::Data<Mutex<State>>,
             session:Session)->Result<String>{
    let mut state_data = data.lock().unwrap();
    state_data.setup(info.video_dir.clone(),info.username.clone(),info.password.clone());
    return Ok("Sucess".to_string());
}
#[derive(Serialize)]
struct empty_struct{

}
pub fn login_html(data:web::Data<Mutex<State>>, session:Session) -> impl Responder{
    println!("ran redirect");
    let mut state_data = data.lock().unwrap();
    let mut html = TERA.render("login.jinja2",&empty_struct{});
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
