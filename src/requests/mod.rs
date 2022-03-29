use std::io::Cursor;

use anyhow::{anyhow, Result};
use log::{error, info};
use reqwest::header::{HeaderMap, HeaderValue, REFERER, USER_AGENT};
use scraper::{Html, Selector};

fn build_cross_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.83 Safari/537.36"));
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://mmzztt.com/photo/"),
    );

    headers
}

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

    let target_dir = "images";
    if let Err(e) = std::fs::create_dir_all(target_dir) {
        return Err(anyhow!("create '{}' failed: {}", target_dir, e));
    }

    for (idx, elem) in (1_u32..).zip(document.select(&selector)) {
        let title = elem.value().attr("alt").unwrap();
        let href = elem.value().attr("data-srcset").unwrap();

        match download(title, href, target_dir) {
            Err(e) => error!("[#{}] {}<{}> {}", idx, title, href, e),
            Ok(size) => {
                if size == 0 {
                    error!("[#{}] {} download failed", idx, title);
                } else {
                    info!(
                        "[#{}] {} downloaded {:.2}KB",
                        idx,
                        title,
                        size as f64 / 1024.0
                    );
                }
            }
        }
    }

    Ok(())
}

fn download(title: &str, url: &str, target_dir: &str) -> Result<u64> {
    let file_ext = url.split(".").last().expect("cant find file ext");
    let file_path = &format!("{}/{}.{}", target_dir, title, file_ext);

    if std::path::Path::new(file_path).exists() {
        return Err(anyhow!("already exists"));
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
        Ok(size) => return Ok(size),
        Err(e) => return Err(anyhow!(e)),
    }
}
