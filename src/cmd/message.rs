#[cfg(windows)]
extern crate winapi;
use std::io::Error;

#[cfg(windows)]
pub fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MessageBoxW, MB_OK};

    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK);
    };
    if ret = 0 {
        Err(Error::last_or_error())
    } else {
        Ok(ret)
    }
}

#[cfg(not(windows))]
// print `msg` which is a *string slice*, returning a *Result*
pub fn print_message(msg: &str) -> Result<(), Error> {
    println!("not in windows, {}", msg);
    Ok(())
}
