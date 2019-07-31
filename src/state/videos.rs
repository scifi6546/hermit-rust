use std::fs::{self};
use serde::{Deserialize,Serialize};
use std::path::Path;
mod thumbnail;
#[derive(Clone)]
pub struct Video{
    path: String,
    pub name: String,
    thumbnail_path: String,
    pub thumbnail_name: String,
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Video_html{
    pub name: String,
    pub url: String,
    pub thumbnail_url: String,
    pub html_url:String,
}
impl Video{
    pub fn getUrl(&self,path_base:String)->String{
        return self.thumbnail_name.clone();
    }
    pub fn getThumb(&self,thumbnail_base: String)->String{
        let mut out:String = thumbnail_base.clone();
        out.push_str(&self.thumbnail_name.clone());
        return out;
    }
    pub fn getVid_html(&self,path_base:String,thumbnail_base:String)->Video_html{
        return Video_html{
            name:self.name.clone(),
            url:self.getUrl(path_base.clone()),
            thumbnail_url: self.getThumb(thumbnail_base),
			html_url:self.getUrl(path_base),
        };
    }
}
fn is_video(path: String)->bool{
    return true;
}
pub fn get_videos(read_dir:String,thumb_dir:String)->Vec<Video>{
    let path=Path::new(&read_dir);
    let thumb_path=Path::new(&thumb_dir);
    assert!(path.is_dir());
    assert!(thumb_path.is_dir());

    println!("looking for videos");
    let path=Path::new(&read_dir);
    let mut out_vid:Vec<Video>=Vec::new();
    //Todo make thumbnail creation run in parallel
    for entry in fs::read_dir(path).unwrap(){
        let entry = entry.unwrap();
        println!("entry: {:?}",entry.path());
        let vid_path_temp:&Path=Path::new(read_dir.as_str());
        let vid_path = vid_path_temp.join(entry.file_name().to_str().unwrap());
        let thumb_info = thumbnail::make_thumb(vid_path.to_str().unwrap().to_string(),thumb_dir.clone()).clone();
        let mut vid = Video{path:"".to_string(),
            name:"".to_string(),
            thumbnail_path: thumb_info[0].clone(), 
            thumbnail_name: thumb_info[1].clone(),
            };
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
        println!("  thumbnail: {}",vid.thumbnail_path);
    }
}
