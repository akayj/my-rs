use clap::{Parser, ValueEnum};

/// Simple program to greet to person.
#[derive(Parser, Debug)]
#[command(
    version = "v0.1",
    author = "Developed by @akayj (Akayj)",
    about = "Simple CLI Application that scratch content from web",
    long_about = None
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

    /// site file
    #[arg(short, long, default_value_t = String::from("sites"))]
    pub site: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Network port to use
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

pub fn parse_args() -> Args {
    // let args = Args::parse();
    let args = Args::parse();

    println!("args: {:?}", args);

    args
}
