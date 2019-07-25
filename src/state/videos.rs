use std::fs::{self};
use std::path::Path;
#[derive(Clone)]
pub struct Video{
    path: String,
    name: String
}
fn is_video(path: String)->bool{
    return true;
}
pub fn get_videos(read_dir:String)->Vec<Video>{
    println!("looking for videos");
    let path=Path::new(&read_dir);
    let mut out_vid:Vec<Video>=Vec::new();
    for entry in fs::read_dir(path).unwrap(){
        let entry = entry.unwrap();
        let mut vid = Video{path:"".to_string(),
            name:"".to_string()};
        vid.path=entry.path().to_str().unwrap().to_string();
        vid.name=entry.path().file_name().unwrap().to_str().unwrap().to_string();
        if is_video(vid.path.clone()){
            out_vid.push(vid);
        }

        println!("file found");
    }
    print_videos(out_vid.clone());
    return out_vid;
}
fn print_videos(videos:Vec<Video>){
    for vid in videos{
        println!("Videos: ");
        println!("  name: {}",vid.name);
        println!("  path: {}",vid.path);
    }
}
