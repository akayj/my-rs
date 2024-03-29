use clap::{Parser, Subcommand};

/// Simple CLI that scratch images from website.
#[derive(Parser, Debug)]
#[command(
    name = "myrs",
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
    Douban {
        #[arg(short, default_value_t = String::from("./images/douban"))]
        target_dir: String,
    },

    /// Wallpaper from wallpaperhub.app
    Wallpaper { size: Option<String> },

    /// Hot images
    Hot {
        /// site list file
        #[arg(short, long, default_value_t = String::from("links"))]
        site: String,

        #[arg(short, default_value_t = String::from("./images/hot"))]
        target_dir: String,
    },

    /// System info
    Info,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    println!("args: {:?}", args);

    args
}
