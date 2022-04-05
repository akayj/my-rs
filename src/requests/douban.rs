use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
// use scraper::{Html, Selector};
use serde::Deserialize;

use super::{build_cross_headers, Douban, Downloader};

#[derive(Debug)]
pub struct LinkMeta {
    pub href: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct MovieLink {
    pub title: String,
    pub url: String,
    pub cover: String,
    pub rate: String,
}

#[derive(Debug, Deserialize)]
pub struct Movies {
    pub subjects: Vec<MovieLink>,
}

impl Douban {
    #[allow(dead_code)]
    pub fn new(site: &str, target_dir: &str) -> Self {
        Self(String::from(site), String::from(target_dir))
    }
}

pub fn fetch_movie_links_json(site: &str) -> Result<Movies> {
    // headers
    let headers = build_cross_headers(site);

    let client = Client::new();
    let resp = client.get(site).headers(headers).send()?.json::<Movies>()?;

    Ok(resp)
}

// pub fn fetch_movie_links(site: &str) -> Result<Vec<LinkMeta>> {
//     // headers
//     let headers = build_cross_headers(site);

//     let client = Client::new();
//     let resp = client.get(site).headers(headers).send()?;

//     if !resp.status().is_success() {
//         return Err(anyhow!("request failed: {:?}", resp.status()));
//     }

//     let text = resp.text()?;

//     let document = Html::parse_document(&text);
//     let selector = Selector::parse(r#"tr.item td:first-child a.nbg img"#).unwrap();

//     let mut links = vec![];
//     for (_, elem) in (1_u32..).zip(document.select(&selector)) {
//         let title = elem.value().attr("alt").unwrap();
//         let href = elem.value().attr("src").unwrap();

//         links.push(LinkMeta {
//             href: String::from(href),
//             title: String::from(title),
//         });
//     }

//     Ok(links)
// }


// fetch douban top movies links
impl Downloader for Douban {
    fn download(&self) -> Result<()> {
        match fetch_movie_links_json(self.0.as_str()) {
            Ok(movies) => {
		let links = movies.subjects;
                if links.is_empty() {
                    return Err(anyhow!("no link found"));
                }

                if let Err(e) = std::fs::create_dir_all(self.1.as_str()) {
                    log::error!("create '{}' failed: {}", self.1, e);
                    return Err(anyhow!("create '{}' failed: {}", self.1, e));
                }

                for link in &links {
		    let rate = link.rate.parse::<f64>().unwrap_or(0_f64);

		    if rate < 6.0 {
			continue;
		    }

                    match super::download(&link.title, &link.cover, self.1.as_str()) {
                        Ok(0) => log::error!("download `{}` failed", link.cover),
                        Ok(-1) => log::warn!("{} already download", link.title),
                        Ok(bytes) => {
                            log::info!("[{} rate: {}] {:.1} KiB <{}>", link.title, rate, bytes as f64 / 1024.0, link.url)
                        }

                        Err(e) => log::error!("download image {} failed: {}", link.cover, e),
                    }
                }

                Ok(())
            }
            Err(e) => {
                log::error!("fetch movie links failed: {}", e);
                return Err(anyhow!("fetch movie links failed: {}", e));
            }
        }
    }
}
