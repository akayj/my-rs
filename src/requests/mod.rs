use std::io::Cursor;

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

pub fn download_images(site: &str) -> Result<(), Box<dyn std::error::Error>> {
    // headers
    let headers = build_cross_headers();

    let client = reqwest::blocking::Client::new();
    let resp = client.get(site).headers(headers).send()?;

    // let resp = reqwest::blocking::get(site)?;
    if !resp.status().is_success() {
        let serr = format!("request failed: {:?}", resp.status());
        let err: Box<_> = String::from(serr).into();
        return Err(err);
    }

    let text = resp.text()?;

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"main > ul > li a.u-thumb-v > img"#).unwrap();

    let target_dir = "images";
    if let Err(e) = std::fs::create_dir_all(target_dir) {
        let error_info = format!("create '{}' failed: {}", target_dir, e);
        let e: Box<_> = String::from(error_info).into();
        return Err(e);
    }

    for (idx, elem) in (1_u32..).zip(document.select(&selector)) {
        // for (idx, elem) in document.select(&selector).enumerate() {
        let title = elem.value().attr("alt").unwrap();
        let href = elem.value().attr("data-srcset").unwrap();

        // let attrs = elem.value().attrs();
        // for (attr_name, attr_val) in attrs {
        //     println!("attr: {} => {}", attr_name, attr_val);
        // }

        // println!("[{}] {}", title, href);

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

fn download(title: &str, url: &str, target_dir: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let file_ext = url.split(".").last().expect("cant find file ext");
    let file_path = &format!("{}/{}.{}", target_dir, title, file_ext);

    if std::path::Path::new(file_path).exists() {
        let err: Box<_> = String::from("already exists").into();
        return Err(err);
    }

    // headers
    let headers = build_cross_headers();

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).headers(headers).send()?;

    if !resp.status().is_success() {
        let serr = format!("request failed: {:?}", resp.status());
        let err: Box<_> = String::from(serr).into();
        return Err(err);
    }

    let mut file = std::fs::File::create(file_path)?;
    let mut content = Cursor::new(resp.bytes()?);
    match std::io::copy(&mut content, &mut file) {
        Ok(size) => return Ok(size),
        Err(e) => return Err(Box::new(e)),
    }
}
