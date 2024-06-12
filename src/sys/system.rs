use sysinfo::{Disks, System};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::sys::cpu_info;
use crate::sys::{battery_info, gpu_info};

// const KB: u64 = 1 << 10;
const MIB: u64 = 1 << 20;
const GIB: u64 = 1 << 30;

// const MB: u64 = 1_000_000;
// const GB: u64 = 1_000_000_000;

pub fn system_info() {
    println!("Hostname: {}", System::host_name().unwrap_or_default());
    println!(
        "Operation System: {} {}",
        System::name().unwrap_or_default(),
        System::os_version().unwrap_or_default(),
        // sys.kernel_version().unwrap_or_default(),
    );

    log::debug!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        log::debug!(
            "Name: {:?}, {:?}, FS: {:?} Free: ({:.1} GiB /{:.1} GiB, {:.2}%)",
            disk.name(),
            disk.mount_point(),
            disk.file_system(),
            human_size(disk.available_space(), GIB),
            human_size(disk.total_space(), GIB),
            (disk.available_space() * 100) as f64 / disk.total_space() as f64,
        );
    }

    // println!("=> networks:");
    // for (interface_name, data) in sys.networks() {
    //     println!(
    //         "{}: {}/{} B",
    //         interface_name,
    //         data.received(),
    //         data.transmitted()
    //     );
    // }

    let mut sys = System::new_all();
    sys.refresh_all();

    log::debug!("=> system:");
    log::debug!(
        "memory: {:.2} GB used of {:.2} GB",
        human_size(sys.used_memory(), MIB),
        human_size(sys.total_memory(), MIB),
    );
    log::debug!(
        "swap: {:.2} GB used of {:.2} GB",
        human_size(sys.used_swap(), MIB),
        human_size(sys.total_swap(), MIB),
    );
}

/// human readable size
fn human_size(x: u64, unit: u64) -> f64 {
    x as f64 / unit as f64
}

pub fn full_info() {
    system_info();

    // cpu_info();

    if let Err(e) = battery_info() {
        log::error!("bad things happened: {}", e);
    }

    // sys::systeminfo();
    match gpu_info() {
        Ok(_) => (),
        Err(e) => println!("get gpu info error: {}", e),
        // Err(_) => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_size() {
        assert_eq!(human_size(1024, 1024), 1f64)
    }
}
