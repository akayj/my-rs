use serde::{Deserialize, Serialize};

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