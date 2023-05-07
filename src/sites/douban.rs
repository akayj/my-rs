use crate::requests::{Douban, Downloader};

pub fn download(target_dir: &str) {
    let url =
        "https://movie.douban.com/j/search_subjects?type=tv&tag=美剧&page_limit=50&page_start=0";
    // let url = "https://movie.douban.com/j/search_subjects?type=movie&tag=热门&page_limit=50&page_start=0";
    // let url = "https://movie.douban.com/j/search_subjects?type=movie&tag=%E7%88%B1%E6%83%85&sort=recommend&page_limit=40&page_start=0";
    // let website = Douban::new(url, "images/douban");
    let website = Douban::new(url, target_dir);
    if let Err(e) = website.download() {
        log::error!("{}", e);
    }
}
