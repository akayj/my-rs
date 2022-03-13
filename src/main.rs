use clap::Parser;
use local_ip_address::local_ip;

mod cmd;
mod sys;

/// Simple program to greet to person.
#[derive(Parser, Debug)]
#[clap(
    version = "v0.1",
    author = "Developed by @akayj (Akayj)",
    about = "An unidentifiable mechanism that helps you bypass GFW"
)]
struct Args {
    /// .toml config file name
    // #[clap(short, long, takes_value = true)]
    #[clap(short, long, default_value_t = String::from("client.toml"), takes_value = true)]
    config: String,
}

fn main() {
    let args = Args::parse();
    println!("args: {:?}", args);

    cmd::run_shell();

    my_area();
    my_addr();
}

fn my_area() {
    // let pi = 3.14156;
    cmd::mybits();
    cmd::print_message("this is rust enabled message").unwrap();

    if let Err(e) = sys::battery_info() {
        println!("error: {}", e);
    }
}

fn my_addr() {
    let my_local_ip = local_ip().unwrap();
    println!("This is my local IP address: {:?}", my_local_ip);
}
