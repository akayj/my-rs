use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use pretty_bytes::converter::convert;

pub fn gpu_info() -> Result<(), anyhow::Error> {
    let nvml = Nvml::init()?;

    let cuda_version = nvml.sys_cuda_driver_version()?;

    // Get the first `Device` (GPU) in the system
    let device = nvml.device_by_index(0)?;

    let name = device.name()?;
    // TODO: this is todo test
    let temp = device.temperature(TemperatureSensor::Gpu)?;
    let mem_info = device.memory_info()?;
    let graphics_clock = device.clock_info(Clock::Graphics)?;
    // let mem_clock = device.clock_info(Clock::Memory)?;
    // let link_gen = device.current_pcie_link_gen()?;
    // let link_width = device.current_pcie_link_width()?;
    // let max_link_gen = device.max_pcie_link_gen()?;
    // let max_link_width = device.max_pcie_link_width()?;
    let cuda_cores = device.num_cores()?;
    let architecture = device.architecture()?;

    let brand = device.brand()?;
    let fan_speed = device.fan_speed(0)?;
    let power_limit = device.enforced_power_limit()?;
    // let encoder_util = device.encoder_utilization()?;

    print!("\n\n");
    println!(
        "GPU: {name} (architecture: {architecture})
CUDA cores: {cuda_cores}
temperature: {temp} C
graphics clock: {graphics_clock} MHz
brand: {brand:?}
fan_speed: {fan_speed}
power limit: {power_limit} W
memory usage is {used_mem} out of an available {total_mem}
        ",
        name = name,
        architecture = architecture,
        cuda_cores = cuda_cores,
        temp = temp,
        graphics_clock = graphics_clock,
        brand = brand,
        fan_speed = fan_speed,
        power_limit = power_limit / 1000,
        used_mem = convert(mem_info.used as f64),
        total_mem = convert(mem_info.total as _)
    );

    if device.is_multi_gpu_board()? {
        println!("This device is on a multi-GPU board.")
    } else {
        println!("This device is on a single-GPU board.")
    }

    println!(
        "System CUDA version: {}.{}",
        cuda_driver_version_major(cuda_version),
        cuda_driver_version_minor(cuda_version),
    );

    Ok(())
}
