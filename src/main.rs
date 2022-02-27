use clap::Parser;

mod cmd;

/// Simple program to greet to person.
#[derive(Parser, Debug)]
#[clap(
    version = "v0.1",
    author = "Developed by @akayj (Akayj)",
    about = "An unidentifiable mechanism that helps you bypass GFW",
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

    if cfg!(not(target_os = "macos")) {
        println!("only support macos now..");
    } else {
        println!("you are running");
    }

    cmd::run_shell();

}
