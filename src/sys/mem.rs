use sysctl::Sysctl;

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
const CTLNAME: &str = "kern.ostype";
#[cfg(any(target_os = "linux", target_os = "andrioid"))]
const CTLNAME: &str = "kernel.ostype";

pub fn print_mem() {
    let ctl = sysctl::Ctl::new(CTLNAME).unwrap();
    println!("Description: {}", ctl.description().unwrap());
    println!("Value: {}", ctl.value_string().unwrap());
}
