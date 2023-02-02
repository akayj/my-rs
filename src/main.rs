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
mod sites;
mod sys;
mod tts;

use std::time::Instant;

use crate::cliargs::parse_args;
use crate::logger::init_log;

fn main() {
    let started = Instant::now();
    let args = parse_args();

    init_log(&args.log_level, &args.log_target);

    sys::full_info();

    sites::douban::download();
    // sites::hot::download(&args.site);

    let flag = emojis::get_by_shortcode("hourglass").unwrap();
    log::info!(target: "app_events", "{} execution cost {:.3} secs", flag, started.elapsed().as_secs_f64());
}
