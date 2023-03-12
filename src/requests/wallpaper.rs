// use crate::requests::build_cross_headers;

use super::Downloader;
use anyhow::Result;

pub struct WallPaper(pub String, pub String);

impl WallPaper {
    pub fn new<S: Into<String>>(site: S, target_dir: S) -> Self {
        Self(site.into(), target_dir.into())
    }
}

fn fetch_image_list() {

}

impl Downloader for WallPaper {

    fn download(&self) -> Result<()> {
        println!("hello WallPaper download");

        // let header = build_cross_headers(self.1.as_str());

        let client = ureq::get(self.0.as_str())
            .set("Referer", "https://www.google.com")
            .call()?
            .into_json()?;

        Ok(())
    }
}
