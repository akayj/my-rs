use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, REFERER, USER_AGENT};

pub use download::{simple_download, DownloadHelper, Downloader};

pub use self::douban::Douban;
pub use self::hot::HotGirl;
pub use self::wallpaper::WallPaper;

mod douban;
mod download;
mod hot;
mod wallpaper;

fn build_cross_headers(refer: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
            AppleWebKit/537.36 (KHTML, like Gecko) \
            Chrome/109.0.0.0 Safari/537.36 Edg/109.0.1518.78",
        ),
    );

    headers.insert(REFERER, HeaderValue::from_str(refer).unwrap());
    headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;\
        q=0.9,image/webp,image/apng,*/*;\
        q=0.8,application/signed-exchange;\
        v=b3;q=0.9",
        ),
    );

    headers
}
