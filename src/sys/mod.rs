use systemstat::{Platform, System};

pub use self::battery::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::cpu::*;
// pub use self::mem::*;
pub use self::gpu::*;
pub use self::system::*;

mod battery;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod cpu;
// mod mem;
pub mod gpu;
#[cfg(target_os = "windows")]
pub mod reg;
mod system;

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

    // if cfg!(target_os = "windows") {
    // if cfg!(windows) {
    //     println!("fetch registry info under windows");
    //     if let Err(e) = reg::query_uninstall_keys(Some("python")) {
    //         println!("found an error {:?}", e);
    //     }
    // }
}
