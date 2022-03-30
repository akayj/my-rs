use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use scraper::{Html, Selector};

use super::build_cross_headers;

#[derive(Debug)]
pub struct LinkMeta {
    pub href: String,
    pub title: String,
}

pub fn fetch_movie_links(site: &str) -> Result<Vec<LinkMeta>> {
    // headers
    let headers = build_cross_headers();

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

pub fn fetch_movie_ads_image(site: &str) {
    match fetch_movie_links(site) {
        Ok(links) => {
            if links.len() == 0 {
                return;
            }

            if let Err(e) = std::fs::create_dir_all(super::IMAGE_DIR) {
                // return Err(anyhow!("create '{}' failed: {}", super::IMAGE_DIR, e));
                log::error!("create '{}' failed: {}", super::IMAGE_DIR, e);
                return;
            }

            for link in &links {
                // log::debug!("found link {:?}", link);
                match super::download(&link.title, &link.href, super::IMAGE_DIR) {
                    Ok(0) => log::error!("download `{}` failed", link.href),
                    Ok(-1) => log::warn!("{} already download", link.title),
                    Ok(bytes) => {
                        log::debug!("download `{}` {:.1} KiB", link.title, bytes as f64 / 1024.0)
                    }

                    Err(e) => log::error!("download image {} failed: {}", link.href, e),
                }
            }
        }
        Err(e) => log::error!("fetch movie links failed: {}", e),
    }
}
