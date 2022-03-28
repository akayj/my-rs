use libc::c_int;
use log::debug;

#[link(name = "cpuid")]
extern "C" {
    fn cpuid_get_total_cpus() -> c_int;
}

extern "C" {
    fn foo();
    fn bar(x: i32) -> i32;
}

pub fn get_cpu_total() {
    let total = unsafe { cpuid_get_total_cpus() as u32 };
    debug!("cpu total count: {}", total);

    unsafe {
        foo();
    }

    let bar_rv = unsafe { bar(22) };
    debug!("bar return value is {}", bar_rv);
}
