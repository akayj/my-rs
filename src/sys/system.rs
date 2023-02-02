use sysinfo::{DiskExt, NetworkExt, System, SystemExt};

// const KB: u64 = 1 << 10;
const MIB: u64 = 1 << 20;
const GIB: u64 = 1 << 30;

// const MB: u64 = 1_000_000;
// const GB: u64 = 1_000_000_000;

pub fn system_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("System name: {}", sys.name().unwrap_or_default());
    println!(
        "System kernel version: {}",
        sys.kernel_version().unwrap_or_default()
    );
    println!(
        "System OS version: {}",
        sys.os_version().unwrap_or_default()
    );
    println!("System host name: {}", sys.host_name().unwrap_or_default());

    log::debug!("=> disks:");
    for disk in sys.disks() {
        log::debug!(
            "Name: {:?}, ({:?}, Free: ({:.1} GiB /{:.1} GiB, {:.2}%)",
            disk.mount_point(),
            disk.type_(),
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
