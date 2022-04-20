use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
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
    // #[allow(dead_code)]
    // pub fn new(site: &str, target_dir: &str) -> Self {
    //     Self(String::from(site), String::from(target_dir))
    // }

    pub fn new<S: Into<String>>(site: S, target_dir: S) -> Self {
        Self(site.into(), target_dir.into())
    }
}

pub fn fetch_movie_links_json(site: &str) -> Result<Movies> {
    // headers
    let headers = build_cross_headers(site);

    let client = Client::new();
    // let resp = client.get(site).headers(headers).send()?.json::<Movies>()?;
    let resp: Movies = client.get(site).headers(headers).send()?.json()?;

    Ok(resp)
}

impl Downloader for Douban {
    fn download(&self) -> Result<()> {
        match fetch_movie_links_json(self.0.as_str()) {
            Ok(Movies { subjects }) => {
                if subjects.is_empty() {
                    return Err(anyhow!("no link found"));
                }

                if let Err(e) = std::fs::create_dir_all(self.1.as_str()) {
                    log::error!("create '{}' failed: {}", self.1, e);
                    return Err(anyhow!("create '{}' failed: {}", self.1, e));
                }

                let mut index = 1u8;
                let mut total_size = 0_i64;
                for link in &subjects {
                    let rate = link.rate.parse::<f64>().unwrap_or(0_f64);

                    if rate < 7.0 {
                        log::warn!("{} rate is too low {}, ignore it", link.title, rate);
                        continue;
                    }

                    // match super::download(&link.title, &link.cover, self.1.as_str()) {
                    match super::simple_download(&link.title, &link.cover, self.1.as_str()) {
                        Ok(0) => log::error!("download `{}` failed", link.cover),
                        Ok(-1) => log::debug!("{} already download", link.title),
                        Ok(bytes) => {
                            log::info!(
                                "[#{}] [{} rate: {}] {:.1} KiB <{}>",
                                index,
                                link.title,
                                rate,
                                bytes as f64 / 1024.0,
                                link.url
                            );

                            index += 1;
                            total_size += bytes;
                        }

                        Err(e) => {
                            log::error!("download image {} failed: {}", link.cover, e);
                        }
                    }
                }

                if index > 1 {
                    log::info!(
                        "已下载 {} 张图片， 共计 {:.2} MiB",
                        index - 1,
                        total_size as f64 / 1_000_000.0
                    );
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
