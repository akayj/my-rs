/* #[macro_use] */
extern crate log;

mod cache;
mod error;
mod ffi;
mod lifetime;
mod notify;
mod requests;
mod serial;
mod sys;
mod tts;

use std::time::Instant;

use clap::Parser;
use env_logger::Target;

use crate::requests::{Douban, Downloader, HotGril};

#[derive(Parser, Debug)]
#[clap(
    version = "v0.1",
    author = "Developed by @akayj (Akayj)",
    about = "Simple CLI Application that scratch content from web"
)]
/// Simple program to greet to person.
struct Args {
    /// .toml config file name
    #[clap(short, long, default_value_t = String::from("client.toml"))]
    config: String,

    /// log level
    #[clap(short, long, default_value_t = String::from("debug"))]
    log_level: String,

    /// log target
    #[clap(short = 't', long, default_value_t = String::from("stderr"))]
    log_target: String,

    /// site file
    #[clap(short, long, default_value_t = String::from("sites"))]
    site: String,
}

pub fn init_log(log_level: &str, log_target: &str) {
    use std::str::FromStr;

    let level = log::LevelFilter::from_str(log_level).unwrap_or_else(|e| {
        println!("parse `{}` error: {}, rollback to *DEBUG*", log_level, e);
        log::LevelFilter::Debug
    });

    let target = match log_target {
        "stderr" => Target::Stderr,
        "stdout" => Target::Stdout,
        _ => Target::Stdout,
    };

    if let Err(e) = env_logger::builder()
        .target(target)
        // .format(|buf, record| {
        //     writeln!(
        //         buf,
        //         "{} [{}] {}",
        //         Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        //         record.level(),
        //         record.args()
        //     )
        // })
        .filter(Some("my_rs"), level)
        .filter(Some("app_events"), log::LevelFilter::Debug)
        .try_init()
    {
        println!("** init log failed: {} **", e);
    }
}

fn main() {
    let started = Instant::now();
    let args = Args::parse();
    log::debug!("args: {:?}", args);

    init_log(&args.log_level, &args.log_target);

    full_info();

    let site_file = &args.site;
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

            // handle every site
            for ref site in sites {
                let dl = HotGril::new(site, &String::from("images/hot"));
                if let Err(e) = dl.download() {
                    log::error!("download images from page `{}` failed: {}", site, e);
                } else {
                    let s = format!("download {} finished", site);
                    let _ = notify::notice(&s);
                }
            }
        }

        Err(e) => log::warn!("read file `{}` failed: {}", site_file, e),
    }

    // Douban
    let url =
        "https://movie.douban.com/j/search_subjects?type=tv&tag=美剧&page_limit=50&page_start=0";
    // let url = "https://movie.douban.com/j/search_subjects?type=movie&tag=热门&page_limit=50&page_start=0";
    // let url = "https://movie.douban.com/j/search_subjects?type=movie&tag=%E7%88%B1%E6%83%85&sort=recommend&page_limit=40&page_start=0";
    let website = Douban::new(url, "images/douban");
    if let Err(e) = website.download() {
        log::error!("{}", e);
    }

    // win::win_main();

    let flag = emojis::get_by_shortcode("hourglass").unwrap();
    log::info!(target: "app_events", "{} execution cost {} secs", flag, started.elapsed().as_secs_f64());
}

fn full_info() {
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

    // if let Err(e) = sys::reg::iter_registry() {
    //     println!("iter registry failed: {}", e);
    // }

    // if let Err(e) = sys::reg::query_uninstall_keys(Some("微信")) {
    // if let Err(e) = sys::reg::query_uninstall_keys(None) {
    //     println!("query multi registry subkeys failed: {}", e);
    // }

    use uuid::Uuid;

    let id = Uuid::new_v4();
    println!("uuid v4: {:?}", id);

    serial::serial_something();
    if let Err(e) = serial::exec() {
        println!("exec nvidia-smi.exe -L failed: {}", e);
    }
}
