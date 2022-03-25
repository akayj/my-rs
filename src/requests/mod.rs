use std::collections::HashMap;
use std::io::Cursor;

use log::{debug, error, info, warn};
use reqwest::header::{HeaderMap, HeaderValue, REFERER, USER_AGENT};
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HTTPBinPostResponse {
    origin: String,
    url: String,
    headers: HashMap<String, String>,
}

pub fn http_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    // Get JSON:
    // let resp = reqwest::blocking::get("https://httpbin.org/ip")?;

    // POST JSON:
    let client = reqwest::blocking::Client::new();
    let resp = client.post("http://httpbin.org/post").json(&map).send()?;

    debug!("resp header: {:?}", resp.headers());

    // let body = resp.text()?;
    // let body = resp.json::<HashMap<String, String>>()?;

    let body = resp.json::<HTTPBinPostResponse>()?;
    debug!("resp: {:#?}", body);
    debug!("resp.header: {:#?}", body.headers);
    debug!("resp.origin: {}", body.origin);
    debug!("resp.url: {}", body.url);
    Ok(())
}

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
    // let site = "https://mmzztt.com/photo/top/";
    // let site = "https://mmzztt.com/photo/";

    // headers
    let headers = build_cross_headers();

    let client = reqwest::blocking::Client::new();
    let resp = client.get(site).headers(headers).send()?;

    // let resp = reqwest::blocking::get(site)?;
    if !resp.status().is_success() {
        panic!("request failed: {:?}", resp.status());
        // return Err("response failed");
    }

    let text = resp.text()?;

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"main > ul > li a.u-thumb-v > img"#).unwrap();

    let target_dir = "images";
    if let Err(e) = std::fs::create_dir_all(target_dir) {
        println!("create '{}' failed: {}", target_dir, e);
        return Ok(());
    }

    for elem in document.select(&selector) {
        let title = elem.value().attr("alt").unwrap();
        let href = elem.value().attr("data-srcset").unwrap();

        // let attrs = elem.value().attrs();
        // for (attr_name, attr_val) in attrs {
        //     println!("attr: {} => {}", attr_name, attr_val);
        // }

        println!("[{}] {}", title, href);

        if let Err(e) = download(title, href, target_dir) {
            println!("download {} error: {}", title, e);
        }
    }

    Ok(())
}

fn download(title: &str, url: &str, target_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_ext = url.split(".").last().expect("cant find file ext");
    let file_path = &format!("{}/{}.{}", target_dir, title, file_ext);

    if std::path::Path::new(file_path).exists() {
        warn!(target: "app_events","Skip {}, already exists!", title);
        return Ok(());
    }

    // headers
    let headers = build_cross_headers();

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).headers(headers).send()?;

    if !resp.status().is_success() {
        error!(target: "app_events","request failed: {:?}", resp.status());
        // return Err("response failed");
    }

    let mut file = std::fs::File::create(file_path)?;
    let mut content = Cursor::new(resp.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    info!(target: "app_events", "Created: {}", file_path);

    Ok(())
}
