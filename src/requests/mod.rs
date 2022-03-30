pub mod books;
pub mod zxzj;

use std::io::Cursor;

use anyhow::{anyhow, Result};
use log::{error, info, warn};
use reqwest::header::{HeaderMap, HeaderValue, REFERER, USER_AGENT};
use scraper::{Html, Selector};

fn build_cross_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.83 Safari/537.36"));
    headers.insert(
        REFERER,
        // HeaderValue::from_static("https://mmzztt.com/photo/"),
        HeaderValue::from_static("https://www.douban.com/"),
    );

    // headers.insert(ACCEPT, HeaderValue::from_static("text/html;image/webp"));

    headers
}

const IMAGE_DIR: &str = "images"; // equals to `const IMAGE_DIR: &'static str = "..."`

pub fn download_images(site: &str) -> Result<()> {
    // headers
    let headers = build_cross_headers();

    let client = reqwest::blocking::Client::new();
    let resp = client.get(site).headers(headers).send()?;

    if !resp.status().is_success() {
        return Err(anyhow!("request failed: {:?}", resp.status()));
    }

    let text = resp.text()?;

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"main > ul > li a.u-thumb-v > img"#).unwrap();

    if let Err(e) = std::fs::create_dir_all(IMAGE_DIR) {
        return Err(anyhow!("create '{}' failed: {}", IMAGE_DIR, e));
    }

    for (idx, elem) in (1_u32..).zip(document.select(&selector)) {
        let title = elem.value().attr("alt").unwrap();
        let href = elem.value().attr("data-srcset").unwrap();

        match download(title, href, IMAGE_DIR) {
            Err(e) => error!("[#{}] {}<{}> {}", idx, title, href, e),
            Ok(size) => match size {
                0 => error!("[#{}] {} download failed", idx, title),
                -1 => warn!("[#{}] {} download skip", idx, title),
                s => info!("[#{}] {} downloaded {:.2}KB", idx, title, s as f64 / 1024.0),
            },
        }
    }

    Ok(())
}

fn download(title: &str, url: &str, target_dir: &str) -> Result<i64> {
    let file_ext = url.split(".").last().expect("cant find file ext");
    let file_path = &format!("{}/{}.{}", target_dir, title, file_ext);

    if std::path::Path::new(file_path).exists() {
        return Ok(-1);
    }

    // headers
    let headers = build_cross_headers();

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
