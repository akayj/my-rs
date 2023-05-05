use anyhow::{anyhow, Result};
use scraper::{Html, Selector};

use super::{build_cross_headers, simple_download, Downloader};

pub struct HotGirl {
    pub site: String,
    pub target_dir: String,
}

impl HotGirl {
    pub fn new<S: Into<String>>(site: S, target_dir: S) -> Self {
        Self {
            site: site.into(),
            target_dir: target_dir.into(),
        }
    }
}

impl Downloader for HotGirl {
    fn download(&self) -> Result<()> {
        // headers
        let headers = build_cross_headers(self.site.as_str());

        let client = reqwest::blocking::Client::new();
        let resp = client.get(self.site.as_str()).headers(headers).send()?;

        if !resp.status().is_success() {
            return Err(anyhow!("request failed: {:?}", resp.status()));
        }

        let text = resp.text()?;

        let document = Html::parse_document(&text);
        let selector = Selector::parse(r#"main > ul > li a.u-thumb-v > img"#).unwrap();

        if let Err(e) = std::fs::create_dir_all(self.target_dir.to_owned()) {
            return Err(anyhow!("create '{}' failed: {}", self.target_dir, e));
        }

        for (idx, elem) in (1u32..).zip(document.select(&selector)) {
            let title = elem.value().attr("alt").unwrap();
            let href = elem.value().attr("data-srcset").unwrap();

            // match download(title, href, self.1.as_str()) {
            match simple_download(title, href, &self.target_dir.as_str()) {
                Err(e) => log::error!("[#{}] {}<{}> {}", idx, title, href, e),
                Ok(size) => match size {
                    0 => log::error!("[#{}] {} download failed", idx, title),
                    -1 => log::warn!("[#{}] {} download skip", idx, title),
                    s => log::info!("[#{}] {} {:.2}KB", idx, title, s as f64 / 1024.0),
                },
            }
        }

        Ok(())
    }
}
