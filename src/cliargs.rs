use std::ops::RangeInclusive;

use clap::{Parser, Subcommand};

/// Simple CLI that scratch images from website.
#[derive(Parser, Debug)]
#[command(
    version = "v0.1",
    author = "Developed by @akayj (Akayj)",
    about = "Simple CLI that scratch images from website"
)]
pub struct Args {
    /// config file name(TOML)
    #[arg(short, long, default_value_t = String::from("client.toml"))]
    pub config: String,

    /// log level
    #[arg(short, long, default_value_t = String::from("debug"))]
    pub log_level: String,

    /// log target
    #[arg(short = 't', long, default_value_t = String::from("stderr"))]
    pub log_target: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    #[command(subcommand)]
    pub(crate) command: SiteCommands,
}

#[derive(Debug, Subcommand)]
pub enum SiteCommands {
    /// fetch movies' images from douban
    Douban { name: Option<String> },

    /// Wallpaper from wallpaperhub.app
    Wallpaper { name: Option<String> },

    /// Hot images
    Hot {
        /// site list file
        #[arg(short, long, default_value_t = String::from("links"))]
        site: String,
    },

    /// System info
    Info,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    println!("args: {:?}", args);

    args
}
