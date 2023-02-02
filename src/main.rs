/* #[macro_use] */
extern crate log;

mod cache;
mod cliargs;
mod error;
mod ffi;
mod lifetime;
mod logger;
mod notify;
mod requests;
mod serial;
mod sys;
mod tts;

use std::time::Instant;

use crate::cliargs::parse_args;
use crate::logger::init_log;
use crate::requests::{Douban, Downloader, HotGril};

fn main() {
    let started = Instant::now();
    let args = parse_args();

    init_log(&args.log_level, &args.log_target);

    log::debug!("args: {:?}", args);

    system_info();

    // let site_file = &args.site;
    // let mut sites = vec![];

    // match cache::read_lines(site_file) {
    //     Ok(lines) => {
    //         // read site list
    //         for line in lines.into_iter().flatten() {
    //             if line.starts_with('#') {
    //                 // log::debug!("ignore site: {}", line);
    //                 continue;
    //             }

    //             let line = line.replace(' ', "");

    //             if line.is_empty() {
    //                 continue;
    //             }

    //             log::debug!("found site: {}", line);
    //             sites.push(line);
    //         }

    //         // handle every site
    //         for ref site in sites {
    //             let dl = HotGril::new(site, &String::from("images/hot"));
    //             if let Err(e) = dl.download() {
    //                 log::error!("download images from page `{}` failed: {}", site, e);
    //             } else {
    //                 let s = format!("download {} finished", site);
    //                 let _ = notify::notice(&s);
    //             }
    //         }
    //     }

    //     Err(e) => log::warn!("read file `{}` failed: {}", site_file, e),
    // }

    // Douban
    // let url =
    //     "https://movie.douban.com/j/search_subjects?type=tv&tag=美剧&page_limit=50&page_start=0";
    // // let url = "https://movie.douban.com/j/search_subjects?type=movie&tag=热门&page_limit=50&page_start=0";
    // // let url = "https://movie.douban.com/j/search_subjects?type=movie&tag=%E7%88%B1%E6%83%85&sort=recommend&page_limit=40&page_start=0";
    // let website = Douban::new(url, "images/douban");
    // if let Err(e) = website.download() {
    //     log::error!("{}", e);
    // }

    let flag = emojis::get_by_shortcode("hourglass").unwrap();
    log::info!(target: "app_events", "{} execution cost {:.3} secs", flag, started.elapsed().as_secs_f64());
}

fn system_info() {
    sys::system_info();
    sys::cpu_info();

    if let Err(e) = sys::battery_info() {
        log::error!("bad things happend: {}", e);
    }

    // sys::systeminfo();
    match sys::gpu::gpu_info() {
        Err(e) => println!("get gpu info error: {}", e),
        Ok(_) => (),
    }

    use uuid::Uuid;

    let id = Uuid::new_v4();
    println!("uuid v4: {:?}", id);

    serial::serial_something();
    if let Err(e) = serial::exec() {
        println!("exec nvidia-smi.exe -L failed: {}", e);
    }
}
