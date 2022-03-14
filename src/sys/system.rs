use sysinfo::{DiskExt, System, SystemExt};

const KB: u64 = 1 << 10;
// const MB: u64 = 1 << 20;
// const GB: u64 = 1 << 30;

const GIB: u64 = 1000_000_000;

pub fn system_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("=> disks:");
    for disk in sys.disks() {
        println!(
            "[{:?}] Type: {:?}, Total: {:.2} GB, Free: {:.2} GB",
            disk.name(),
            disk.type_(),
            human_size(disk.total_space(), GIB),
            human_size(disk.available_space(), GIB),
        );
    }

    // sys.disks().iter().for_each(|disk| {
    //     println!(
    //         "[{:?}] Type: {:?}, Total: {:.2} GB, Free: {:.2} GB",
    //         disk.name(),
    //         disk.type_(),
    //         human_size(disk.total_space(), GIB),
    //         human_size(disk.available_space(), GIB),
    //     );
    // });

    // println!("=> networks:");
    // for (interface_name, data) in sys.networks() {
    //     println!(
    //         "{}: {}/{} B",
    //         interface_name,
    //         data.received(),
    //         data.transmitted()
    //     );
    // }

    println!("=> system:");
    println!(
        "\ttotal memory: {:.2} MB",
        human_size(sys.total_memory(), KB)
    );
    println!("\tused memory: {:.2} MB", human_size(sys.used_memory(), KB));
    println!("\ttotal swap: {:.2} MB", human_size(sys.total_swap(), KB));
    println!("\tused swap: {:.2} MB", human_size(sys.used_swap(), KB));
}

/// humanize size
fn human_size(x: u64, unit: u64) -> f64 {
    x as f64 / unit as f64
}
