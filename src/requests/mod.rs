pub mod douban;
pub mod hot;
pub mod traits;

use std::io::{Cursor, Read};

use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, REFERER, USER_AGENT};

fn build_cross_headers(refer: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        // Chrome:
        // "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.60 Safari/537.36"
        // Firefox:
        // "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:98.0) Gecko/20100101 Firefox/98.0"
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:98.0) Gecko/20100101 Firefox/98.0",
        ),
    );

    headers.insert(REFERER, HeaderValue::from_str(refer).unwrap());
    // headers.insert(ACCEPT, HeaderValue::from_static("text/html;image/webp"));

    headers
}

pub fn simple_download(title: &str, url: &str, target_dir: &str) -> Result<i64> {
    let file_ext = url.split('.').last().expect("cant find file ext");
    let file_path = &format!("{}/{}.{}", target_dir, title, file_ext);

    if std::path::Path::new(file_path).exists() {
        return Ok(-1);
    }

    let resp = ureq::get(url)
        .set(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:98.0) Gecko/20100101 Firefox/98.0",
        )
        .set("Referer", url)
        .call()?;

    if resp.status() != 200 {
        return Err(anyhow!("request failed: {:?}", resp.status()));
    }

    let mut file = std::fs::File::create(file_path)?;
    // FIXME: larger than 10 megabytes will caurse an error. Try use `into_reader` instead.
    // let bs = resp.into_string()?;

    let mut buf = Vec::new();
    resp.into_reader().take(10_000_000).read_to_end(&mut buf)?;

    let mut content = Cursor::new(buf);
    match std::io::copy(&mut content, &mut file) {
        Ok(size) => Ok(size as i64),
        Err(e) => Err(anyhow!(e)),
    }
}

pub struct Douban(pub String, pub String);
pub struct HotGril(pub String, pub String);

pub trait Downloader {
    fn download(&self) -> Result<()>;
}
