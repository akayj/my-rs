use crate::requests::{Downloader, WallPaper};

pub fn download(target_dir: &str) {
    let url = "https://wallpaperhub.app/";
    let website = WallPaper::new(url, target_dir);
    if let Err(e) = website.download() {
        log::error!("{}", e);
    }
}
