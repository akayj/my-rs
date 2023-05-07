use crate::requests::{Downloader, HotGirl};
use crate::{cache, notify};

pub fn download(site_file: &str) {
    let mut sites = vec![];

    match cache::read_lines(site_file) {
        Ok(lines) => {
            // read site list
            for line in lines.into_iter().flatten() {
                if line.starts_with('#') {
                    // log::debug!("ignore site: {}", line);
                    continue;
                }

                let line = line.replace(' ', "");

                if line.is_empty() {
                    continue;
                }

                log::debug!("found site: {}", line);
                sites.push(line);
            }

            // handle every sites
            for ref site in sites {
                let dl = HotGirl::new(site, &String::from("images/hot"));
                if let Err(e) = dl.download() {
                    log::error!("download images from page `{}` failed: {}", site, e);
                } else {
                    let s = format!("download {} finished", site);
                    let _ = notify::notice(&s);
                }
            }
        }

        Err(e) => log::error!("read file `{}` failed: {}", site_file, e),
    }
}
