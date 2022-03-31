use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use scraper::{Html, Selector};

use super::{build_cross_headers, Douban, Downloader};

#[derive(Debug)]
pub struct LinkMeta {
    pub href: String,
    pub title: String,
}

pub fn fetch_movie_links(site: &str) -> Result<Vec<LinkMeta>> {
    // headers
    let headers = build_cross_headers(site);

    let client = Client::new();
    let resp = client.get(site).headers(headers).send()?;

    if !resp.status().is_success() {
        return Err(anyhow!("request failed: {:?}", resp.status()));
    }

    let text = resp.text()?;

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"tr.item td:first-child a.nbg img"#).unwrap();

    let mut links = vec![];
    for (_, elem) in (1_u32..).zip(document.select(&selector)) {
        let title = elem.value().attr("alt").unwrap();
        let href = elem.value().attr("src").unwrap();

        links.push(LinkMeta {
            href: String::from(href),
            title: String::from(title),
        });
    }

    Ok(links)
}

// fetch douban top movies links
impl Downloader for Douban {
    fn download(&self) -> Result<()> {
        match fetch_movie_links(self.0.as_str()) {
            Ok(links) => {
                if links.len() == 0 {
                    return Ok(());
                }

                if let Err(e) = std::fs::create_dir_all(self.1.as_str()) {
                    log::error!("create '{}' failed: {}", self.1, e);
                    return Err(anyhow!("create '{}' failed: {}", self.1, e));
                }

                for link in &links {
                    match super::download(&link.title, &link.href, self.1.as_str()) {
                        Ok(0) => log::error!("download `{}` failed", link.href),
                        Ok(-1) => log::warn!("{} already download", link.title),
                        Ok(bytes) => {
                            log::info!("download `{}` {:.1} KiB", link.title, bytes as f64 / 1024.0)
                        }

                        Err(e) => log::error!("download image {} failed: {}", link.href, e),
                    }
                }

                Ok(())
            }
            Err(e) => {
                log::error!("fetch movie links failed: {}", e);
                Ok(())
            }
        }
    }
}
