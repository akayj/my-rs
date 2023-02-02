use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    version = "v0.1",
    author = "Developed by @akayj (Akayj)",
    about = "Simple CLI Application that scratch content from web"
)]
/// Simple program to greet to person.
pub struct Args {
    /// .toml config file name
    #[clap(short, long, default_value_t = String::from("client.toml"))]
    pub config: String,

    /// log level
    #[clap(short, long, default_value_t = String::from("debug"))]
    pub log_level: String,

    /// log target
    #[clap(short = 't', long, default_value_t = String::from("stderr"))]
    pub log_target: String,

    /// site file
    #[clap(short, long, default_value_t = String::from("sites"))]
    pub site: String,
}

pub fn parse_args() -> Args {
    // let args = Args::parse();
    Args::parse()
}
