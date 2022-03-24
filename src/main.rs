#[macro_use]
extern crate log;

use std::collections::HashMap;

use clap::Parser;
// use local_ip_address::local_ip;

mod cache;
#[cfg(target_os = "macos")]
mod camera;
mod cmd;
mod ds;
mod rand;
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

    if let Err(e) = env_logger::builder().filter_level(level).try_init() {
        println!("init log failed: {}", e);
    }
}

fn main() {
    let args = Args::parse();
    debug!("args: {:?}", args);

    init_log(&args.log_level);

    debug!("starting up");

    if let Err(e) = http_request() {
        debug!("failed do http request: {}", e);
    }

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
    cache::which_word();

    video::read_file("./Cargo.toml");
    // video::parse_mp4("/Users/yj/我的电影/adam_project.mp4");

    let s1 = "hello";
    let s2 = "hello world";
    let result = longest(s1, s2);
    debug!(
        "longest string between '{}' and '{}' is: '{}'",
        s1, s2, result
    );

    // camera::run();
}

fn my_area() {
    cmd::mybits();
    cmd::print_message("this is rust enabled message").unwrap();

    if let Err(e) = sys::battery_info() {
        error!("error: {}", e);
    }

    sys::system_info();
    sys::cpu_info();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn http_request() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?;
    debug!("resp header: {:?}", resp.headers());

    let body = resp.json::<HashMap<String, String>>()?;
    debug!("resp: {:#?}", body);
    Ok(())
}

// fn my_addr() {
//     match local_ip() {
//         Ok(ip) => println!("This is my local IP address: {:?}", ip),
//         _ => println!("local IP is unknown."),
//     }
// }
