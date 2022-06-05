// mod battery;
mod cpu;
// mod mem;
mod system;

// pub use self::battery::*;
pub use self::cpu::*;
// pub use self::mem::*;
pub use self::system::*;

use systemstat::{Platform, System};

pub fn systeminfo() {
    let sys = System::new();

    match sys.mounts() {
        Ok(mounts) => {
            println!("\nMounts:");
            for mount in mounts.iter() {
                println!(
                    "{} --{}---> {} (available {} of {})",
                    mount.fs_mounted_from,
                    mount.fs_type,
                    mount.fs_mounted_on,
                    mount.avail,
                    mount.total
                );
            }
        }

        Err(e) => println!("\nMounts: error: {}", e),
    }

    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetworks:");
            for netif in netifs.values() {
                println!("{} ({:?})", netif.name, netif.addrs);
            }
        }
        Err(e) => println!("\nNetworks: error: {}", e),
    }

    match sys.battery_life() {
        Ok(battery) => print!(
            "\nBattery: {}%, {}h{}m remaining",
            battery.remaining_capacity * 100.0,
            battery.remaining_time.as_secs() / 3600,
            battery.remaining_time.as_secs() % 60
        ),
        Err(e) => println!("\nBattery: error: {}", e),
    }

    match sys.on_ac_power() {
        Ok(power) => println!(", AC power: {}", power),
        Err(e) => println!(", AC power: error: {}", e),
    }
}
