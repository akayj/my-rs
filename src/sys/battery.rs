extern crate battery;

pub fn battery_info() -> Result<(), battery::Error> {
    let manager = battery::Manager::new()?;

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;

        println!("Battery #{}:", idx);
        println!("Vendor: {}", battery.vendor().unwrap_or_default());
        println!("Model: {}", battery.model().unwrap_or_default());
        println!("State: {}", battery.state().to_string());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("State of charging: {:?}", battery.state_of_charge());
        // println!("battery: {:?}", battery);
        println!("");
    }

    Ok(())
}
