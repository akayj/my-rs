mod douban;
mod download;
mod hot;
mod wallpaper;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, REFERER, USER_AGENT};

fn build_cross_headers(refer: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        // Chrome:
        // "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.60 Safari/537.36"
        // Firefox:
        // "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:98.0) Gecko/20100101 Firefox/98.0"
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36 Edg/109.0.1518.78",
        ),
    );

    headers.insert(REFERER, HeaderValue::from_str(refer).unwrap());
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));

    headers
}

pub use self::douban::Douban;
pub use self::hot::HotGirl;
pub use self::wallpaper::WallPaper;
pub use download::{simple_download, DownloadHelper, Downloader};
