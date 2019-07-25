use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct User{
    username: String,
    passwd: String
}
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct VideoConf{
    pub video_path: String,
    pub thumbnails: String,
    pub playlists: Vec<u8>
}
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Config{
    pub users:Vec<User>,
    pub videos:VideoConf

}
fn get_config()->std::io::Result<Config>{
    println!("ran?");
    
    let mut file=File::open("config.json")?;
    let mut string = String::new();
    file.read_to_string(&mut string);
    let config:Config = serde_json::from_str(&string).unwrap();
    Ok(config)
}
fn print_config(input: Config){
    println!("Users: ");
    for user in input.users{
        println!("   username: {}",user.username);
        println!("   password: {}",user.passwd);
    }
    println!("Video: ");
    println!("  video_path: {}",input.videos.video_path);
    println!("  thumbnail_path: {}",input.videos.thumbnails);
}
pub fn load_config()->Config{
    let result=get_config();
    let config_out=result.unwrap();
    print_config(config_out.clone());
    return config_out
}
