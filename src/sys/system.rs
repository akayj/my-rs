use log::debug;
use sysinfo::{DiskExt, System, SystemExt};

// const KB: u64 = 1 << 10;
const MB: u64 = 1 << 20;
// const GB: u64 = 1 << 30;

const GIB: u64 = 1000_000_000;

pub fn system_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    debug!("=> disks:");
    for disk in sys.disks() {
        debug!(
            // "[{:?}] Type: {:?}, Total: {:.2} GB, Free: {:.2} GB",
            // disk.name(),
            "Type: {:?}, Total: {:.2} GB, Free: {:.2} GB",
            disk.type_(),
            human_size(disk.total_space(), GIB),
            human_size(disk.available_space(), GIB),
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

    debug!("=> system:");
    debug!(
        "total memory: {:.2} GB, used mem: {:.2} GB",
        human_size(sys.total_memory(), MB),
        human_size(sys.used_memory(), MB),
    );
    debug!(
        "total swap: {:.2} GB, used: {:.2} GB",
        human_size(sys.total_swap(), MB),
        human_size(sys.used_swap(), MB),
    );
}

/// human readable size
fn human_size(x: u64, unit: u64) -> f64 {
    x as f64 / unit as f64
}
