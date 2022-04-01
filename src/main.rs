// #[macro_use]
extern crate log;

mod cache;
mod error;
mod ffi;
mod notify;
mod requests;
mod sys;

use std::time::Instant;

// use chrono::Local;
use clap::Parser;
use env_logger::Target;

use crate::requests::{Douban, Downloader, HotGril};

#[derive(Parser, Debug)]
#[clap(
    version = "v0.1",
    author = "Developed by @akayj (Akayj)",
    about = "An unidentifiable mechanism that helps you bypass GFW"
)]
/// Simple program to greet to person.
struct Args {
    /// .toml config file name
    // #[clap(short, long, takes_value = true)]
    #[clap(short, long, default_value_t = String::from("client.toml"), takes_value = true)]
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

fn init_log(log_level: &str, log_target: &str) {
    let level = match log_level {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Debug,
    };

    let target = match log_target {
        "stderr" => Target::Stderr,
        "stdout" => Target::Stdout,
        _ => Target::Stdout,
    };

    if let Err(e) = env_logger::builder()
        .target(target)
        // .filter_level(level)
        // .filter(Some("scraper"), log::LevelFilter::Error)
        // .filter(Some("html5ever"), log::LevelFilter::Error)
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

    log::debug!("starting up");

    full_info();

    let site_file = &args.site;
    let mut sites = vec![];
    match cache::read_lines(site_file) {
        Ok(lines) => {
            // read site list
            for line in lines {
                if let Ok(site) = line {
                    if !site.starts_with("#") {
                        log::debug!("found site: {}", site);
                        sites.push(site);
                    } else {
                        log::warn!("ignore site: {}", site);
                    }
                }
            }

            // handle every site
            for ref site in sites {
                let hot = HotGril(site.to_string(), String::from("images/hot"));
                if let Err(e) = hot.download() {
                    log::error!("download images from page `{}` failed: {}", site, e);
                } else {
                    let s = format!("download {} finished", site);
                    let _ = notify::notice(&s);
                }
            }
        }

        Err(e) => log::error!("read file `{}` failed: {}", site_file, e),
    }

    // Douban
    let website = Douban(
        String::from("https://movie.douban.com/chart"),
        String::from("images/douban"),
    );
    let _ = website.download();

    error::error_print();

    log::info!(target: "app_events",
	       "execution cost {:.2} secs",
	       started.elapsed().as_secs_f64());
}

fn full_info() {
    if let Err(e) = sys::battery_info() {
        log::error!("error: {}", e);
    }

    sys::system_info();
    sys::cpu_info();
}
