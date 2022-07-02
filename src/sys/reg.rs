use std::io;
use winreg::enums::*;
use winreg::RegKey;
use winreg::HKEY;

const REGKEY_UNINSTALL: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
const REGKEY_UNINSTALL2: &str =
    "Software\\Wow6432node\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

// pub fn iter_registry() -> io::Result<()> {
//     println!("Reading some system info...");

//     let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

//     let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
//     let pf: String = cur_ver.get_value("ProgramFilesDir")?;
//     let dp: String = cur_ver.get_value("DevicePath")?;
//     println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);

//     Ok(())
// }

pub fn query_regitems(key: HKEY, sub_key: &str, name: Option<&str>) -> io::Result<()> {
    let hklm = RegKey::predef(key);
    let keys = hklm.open_subkey(sub_key)?;

    for i in keys.enum_keys().map(|x| x.unwrap()) {
        let leaf_path = format!("{}\\{}", sub_key, i);
        let leaf_key = hklm.open_subkey(&leaf_path)?;

        let display_name: String = leaf_key.get_value("DisplayName").unwrap_or_default();

        if name != None && !display_name.contains(name.unwrap()) {
            continue;
        }

        let install_location: String = leaf_key.get_value("InstallLocation").unwrap_or_default();
        let install_source: String = leaf_key.get_value("InstallSource").unwrap_or_default();
        let uninstall_string: String = leaf_key.get_value("UninstallString").unwrap_or_default();

        if display_name != "" {
            println!(
                "[{display_name}]
            hkey: {leaf_path}
            install: {install_location}
            source: {install_source}
            uninstall: {uninstall_string}",
            );
        }
    }

    Ok(())
}

pub fn query_uninstall_keys(name: Option<&str>) -> io::Result<()> {
    if let Err(e) = query_regitems(HKEY_LOCAL_MACHINE, REGKEY_UNINSTALL, name) {
        println!(
            "query HKEY_LOCAL_MACHINE\\{} failed: {}",
            REGKEY_UNINSTALL, e
        );
    }

    if let Err(e) = query_regitems(HKEY_LOCAL_MACHINE, REGKEY_UNINSTALL2, name) {
        println!(
            "query HKEY_LOCAL_MACHINE\\{} failed: {}",
            REGKEY_UNINSTALL2, e
        );
    }

    if let Err(e) = query_regitems(HKEY_CURRENT_USER, REGKEY_UNINSTALL, name) {
        println!(
            "query HKEY_CURRENT_USER\\{} failed: {}",
            REGKEY_UNINSTALL, e
        );
    }

    Ok(())
}
