use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub fn win_main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(w!("<html>hello</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello");

    unsafe {
        let event = CreateEventW(std::ptr::null(), true, false, None)?;
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
    }

    Ok(())
}
