use std::io;
use std::fs::{self,DirEntry};
use std::path::Path;
#[derive(Clone)]
pub struct Video{
    path: &'a Path,
    name: String
}
pub fn get_videos(read_dir:String)->Vec<Video>{
    println!("looking for videos");
    let path=Path::new(&read_dir);
    let mut out_vid:Vec<Video>=Vec::new();
    for entry in fs::read_dir(path).unwrap(){
        let entry = entry.unwrap();
        let mut vid = Video{path:"".to_string(),
            name:"".to_string()};
        vid.path=entry.path();
        vid.name=entry.path().to_str().unwrap().to_string();

        out_vid.push(vid);

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
