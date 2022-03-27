use notify_rust::Notification;

pub fn notice(msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("Firefox News")
        .body(msg)
        .icon("firefox")
        .show()?;

    Ok(())
}
