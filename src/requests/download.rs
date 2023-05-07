use std::fs::File;
use std::io::{self, Cursor, Read};
use std::path::Path;

use anyhow::{anyhow, Result};

const USER_AGENT: &str = "\
Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
AppleWebKit/537.36 (KHTML, like Gecko) \
Chrome/109.0.0.0 Safari/537.36 Edg/109.0.1518.78";

const HEADER_ACCEPT: &str = "\
text/html,application/xhtml+xml,application/xml;\
q=0.9,image/webp,image/apng,*/*;\
q=0.8,application/signed-exchange;\
v=b3;\
q=0.9";

pub fn simple_download(title: &str, url: &str, target_dir: &str) -> Result<i64> {
    let file_ext = url.split('.').last().expect("cant find file ext");
    let file_path = &format!("{}/{}.{}", target_dir, title, file_ext);

    if Path::new(file_path).exists() {
        return Ok(-1);
    }

    let resp = ureq::get(url)
        .set("Accept", HEADER_ACCEPT)
        .set("User-Agent", USER_AGENT)
        .call()?;

    if resp.status() != 200 {
        return Err(anyhow!("request failed: {:?}", resp.status()));
    }

    // FIXME: will lost data while file size is larger than 10 megabytes. Try use `into_reader` instead.
    let len: usize = resp.header("Content-Length").unwrap().parse()?;
    let mut buf: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader().take(10_000_000).read_to_end(&mut buf)?;

    let mut file = File::create(file_path)?;
    let mut content = Cursor::new(buf);
    match io::copy(&mut content, &mut file) {
        Ok(size) => Ok(size as i64),
        Err(e) => Err(anyhow!(e)),
    }
}

pub trait Downloader {
    fn download(&self) -> Result<()>;
}

pub trait DownloadHelper {
    fn help(&self) -> Result<()>;
}
