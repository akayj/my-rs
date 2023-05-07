// use crate::requests::build_cross_headers;

use super::Downloader;
use anyhow::Result;

pub struct WallPaper {
    pub site: String,
    pub target_dir: String,
}

impl WallPaper {
    pub fn new<S: Into<String>>(site: S, target_dir: S) -> Self {
        Self {
            site: site.into(),
            target_dir: target_dir.into(),
        }
    }
}

fn fetch_image_list() {}

impl Downloader for WallPaper {
    fn download(&self) -> Result<()> {
        // let header = build_cross_headers(self.1.as_str());

        let resp = ureq::get(self.site.as_str())
            .set("Referer", "https://www.google.com")
            .call()?;

        println!("wallpaper request status: {}", resp.status());

        Ok(())
    }
}
