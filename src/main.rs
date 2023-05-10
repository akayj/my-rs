/* #[macro_use] */
extern crate log;

use std::thread;
use std::time::{Duration, Instant};
use std::{cmp::min, fmt::Write};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::cliargs::{parse_args, SiteCommands};
use crate::logger::init_log;

mod cache;
mod cliargs;
mod lifetime;
mod logger;
mod notify;
mod requests;
mod sites;
mod sys;

fn main() {
    let started = Instant::now();
    let args = parse_args();

    init_log(&args.log_level, &args.log_target);

    match &args.command {
        SiteCommands::Info => {
            // sys::full_info();
            sys::systeminfo()
        }

        SiteCommands::Hot { site, target_dir } => {
            println!("hot site is {site:?}");
            sites::hot::download(site, target_dir);
        }

        SiteCommands::Douban { target_dir } => {
            println!("douban target dir is {target_dir:?}");
            sites::douban::download(target_dir);
        }

        SiteCommands::Wallpaper { size } => {
            println!("request wallpaper size is {size:?}");
            sites::wallpaper::download("E://images/wallpapers");
        }
    }

    let flag = emojis::get_by_shortcode("hourglass").unwrap();
    log::info!(target: "app_events", "{} execution cost {:.3} secs", flag, started.elapsed().as_secs_f64());

    indicator();
}

fn indicator() {
    let mut downloaded = 0;
    let total_size = 1024 * 1024;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_size} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 1024 * 50, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}
