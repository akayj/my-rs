use chrono::Local;
use env_logger::Target;
use std::io::Write;

pub fn init_log(log_level: &str, log_target: &str) {
    use std::str::FromStr;

    let level = log::LevelFilter::from_str(log_level).unwrap_or_else(|e| {
        println!("parse `{}` error: {}, rollback to *DEBUG*", log_level, e);
        log::LevelFilter::Debug
    });

    let target = match log_target {
        "stderr" => Target::Stderr,
        "stdout" => Target::Stdout,
        _ => Target::Stdout,
    };

    if let Err(e) = env_logger::builder()
        .target(target)
        // .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("myrs"), level)
        .filter(Some("app_events"), log::LevelFilter::Debug)
        .try_init()
    {
        println!("** init log failed: {} **", e);
    }
}
