pub mod douban;
pub mod hot;
pub mod traits;

use std::io::Cursor;

use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, REFERER, USER_AGENT};

fn build_cross_headers(refer: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.83 Safari/537.36"));
    headers.insert(REFERER, HeaderValue::from_str(refer).unwrap());

    // headers.insert(ACCEPT, HeaderValue::from_static("text/html;image/webp"));

    headers
}

fn download(title: &str, url: &str, target_dir: &str) -> Result<i64> {
    let file_ext = url.split(".").last().expect("cant find file ext");
    let file_path = &format!("{}/{}.{}", target_dir, title, file_ext);

    if std::path::Path::new(file_path).exists() {
        return Ok(-1);
    }

    // headers
    let headers = build_cross_headers(url);

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).headers(headers).send()?;

    if !resp.status().is_success() {
        return Err(anyhow!("request failed: {:?}", resp.status()));
    }

    let mut file = std::fs::File::create(file_path)?;
    let mut content = Cursor::new(resp.bytes()?);
    match std::io::copy(&mut content, &mut file) {
        Ok(size) => return Ok(size as i64),
        Err(e) => return Err(anyhow!(e)),
    }
}

// pub enum SiteType {
//     Douban,
//     HotGril,
//     Null,
// }
pub struct Douban(pub String, pub String);
pub struct HotGril(pub String, pub String);

// pub struct WebSite {
//     pub downloader: Box<dyn Downloader>,
//     pub site: String,
//     pub target_dir: String,
// }

// impl WebSite {
//     pub fn new(downloader: Box<dyn Downloader>, site: &str, target_dir: &str) -> Self {
//         Self {
//             downloader,
//             site: String::from(site),
//             target_dir: String::from(target_dir),
//         }
//     }
// }

pub trait Downloader {
    fn download(&self) -> Result<()>;
}
