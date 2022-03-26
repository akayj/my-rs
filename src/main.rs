// #[macro_use]
extern crate log;

use std::time::Instant;

use clap::Parser;
use env_logger::Target;
use log::{debug, error, info};
// use local_ip_address::local_ip;

mod cache;
#[cfg(target_os = "macos")]
mod camera;
mod cmd;
mod ds;
mod rand;
mod requests;
mod sys;
mod video;

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
        .filter(Some("scraper"), log::LevelFilter::Error)
        .filter(Some("html5ever"), log::LevelFilter::Error)
        .filter(Some("my_rs"), level)
        .filter(Some("app_events"), log::LevelFilter::Debug)
        .try_init()
    {
        println!("init log failed: {}", e);
    }
}

fn main() {
    let started = Instant::now();

    let args = Args::parse();
    debug!("args: {:?}", args);

    init_log(&args.log_level, &args.log_target);

    debug!("starting up");

    full_info();

    // if let Err(e) = requests::http_request() {
    //     error!("failed do http request: {}", e);
    // }

    let sites = vec!["https://mmzztt.com/photo/", "https://mmzztt.com/photo/top/"];
    // let sites = vec![];
    for site in sites {
        if let Err(e) = requests::download_images(site) {
            error!("download images from page `{}` failed: {}", site, e);
        }
    }

    info!(target: "app_events", "execution cost {:.2} secs", started.elapsed().as_secs_f64());
}

fn full_info() {
    #[cfg(target_os = "windows")]
    cmd::print_message("this is rust enabled message").unwrap();

    if let Err(e) = sys::battery_info() {
        error!("error: {}", e);
    }

    sys::system_info();
    sys::cpu_info();
}

// fn my_addr() {
//     match local_ip() {
//         Ok(ip) => println!("This is my local IP address: {:?}", ip),
//         _ => println!("local IP is unknown."),
//     }
// }
