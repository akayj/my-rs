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
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .set(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36 Edg/109.0.1518.78",
        )
        // .set("Referer", url)
        .call()?;

    if resp.status() != 200 {
        return Err(anyhow!("request failed: {:?}", resp.status()));
    }

    // FIXME: larger than 10 megabytes will caurse an error. Try use `into_reader` instead.
    let mut buf = Vec::new();
    resp.into_reader().take(10_000_000).read_to_end(&mut buf)?;

    let mut file = std::fs::File::create(file_path)?;
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

pub trait DownloadHelper {
    fn help(&self) -> Result<()>;
}
