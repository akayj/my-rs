use crate::requests::{WallPaper,Downloader};

pub fn download() {
    let url = "https://wallhaven.cc/w/yxx3kd";
    let website = WallPaper::new(url, "image/wallpaper");
    if let Err(e) = website.download() {
        log::error!("{}", e);
    }
}
