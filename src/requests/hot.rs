use anyhow::{anyhow, Result};
use scraper::{Html, Selector};

use super::{build_cross_headers, simple_download, Downloader};

pub struct HotGril(pub String, pub String);

impl HotGril {
    pub fn new<S: Into<String>>(site: S, target_dir: S) -> Self {
        Self(site.into(), target_dir.into())
    }
}

impl Downloader for HotGril {
    fn download(&self) -> Result<()> {
        // headers
        let headers = build_cross_headers(self.1.as_str());

        let client = reqwest::blocking::Client::new();
        let resp = client.get(self.0.as_str()).headers(headers).send()?;

        if !resp.status().is_success() {
            return Err(anyhow!("request failed: {:?}", resp.status()));
        }

        let text = resp.text()?;

        let document = Html::parse_document(&text);
        let selector = Selector::parse(r#"main > ul > li a.u-thumb-v > img"#).unwrap();

        if let Err(e) = std::fs::create_dir_all(self.1.to_owned()) {
            return Err(anyhow!("create '{}' failed: {}", self.1, e));
        }

        for (idx, elem) in (1u32..).zip(document.select(&selector)) {
            let title = elem.value().attr("alt").unwrap();
            let href = elem.value().attr("data-srcset").unwrap();

            // match download(title, href, self.1.as_str()) {
            match simple_download(title, href, self.1.as_str()) {
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
