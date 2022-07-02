use serde::{Deserialize, Serialize};
use subprocess::Exec;

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

pub fn serial_something() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);
}

pub fn exec() -> anyhow::Result<()> {
    let exit_status = Exec::cmd("C:\\Windows\\system32\\nvidia-smi.exe")
        .arg("-L")
        .join()?;

    println!("nvidia-smi.exe -L returns {:?}", exit_status);

    Ok(())
}
