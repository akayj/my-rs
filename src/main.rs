// #[macro_use]
extern crate log;

use std::time::Instant;

use clap::Parser;
use log::{debug, error};
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
}

fn init_log(log_level: &str) {
    let level = match log_level {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Debug,
    };

    if let Err(e) = env_logger::builder()
        // .filter_level(level)
        .filter(Some("scraper"), log::LevelFilter::Error)
        .filter(Some("html5ever"), log::LevelFilter::Error)
        .filter(Some("app_events"), level)
        .try_init()
    {
        println!("init log failed: {}", e);
    }
}

fn main() {
    let started = Instant::now();

    let args = Args::parse();
    debug!("args: {:?}", args);

    init_log(&args.log_level);

    debug!("starting up");

    // if let Err(e) = requests::http_request() {
    //     error!("failed do http request: {}", e);
    // }

    // requests::parse_html();

    cmd::run_shell();

    my_area();
    // my_addr();

    sys::battle();
    sys::moves();

    ds::new_hashmap();
    ds::raii();
    ds::drop_struct_unit();
    ds::refs();

    cache::cache();
    cache::list_dirs();

    video::read_file("./Cargo.toml");
    // video::parse_mp4("/Users/yj/我的电影/adam_project.mp4");

    // camera::run();

    if let Err(e) = requests::download_images("https://mmzztt.com/photo/") {
        error!("fetch golang download page failed: {}", e);
    }

    if let Err(e) = requests::download_images("https://mmzztt.com/photo/top/") {
        error!("fetch golang download page failed: {}", e);
    }

    debug!(target: "app_events", "execution cost {:.2} secs", started.elapsed().as_secs_f64());
}

fn my_area() {
    cmd::mybits();

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
