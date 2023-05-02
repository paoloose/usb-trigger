use crate::usb_event::USBEvent;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

pub trait EventsIter: Iterator<Item = USBEvent> { }

pub trait Watcher {
    fn new() -> Result<Self, std::io::Error>
        where Self: Sized;
    fn upcoming(self: Box<Self>) -> Box<dyn EventsIter>;
}

pub fn create_watcher() -> Box<dyn Watcher> {
    #[cfg(target_os = "linux")]
    return Box::new(linux::LinuxWatcher::new().unwrap());
    #[cfg(target_os = "windows")]
    return Box::new(windows::WindowsWatcher::new().unwrap());
}

#[cfg(not(any(target_os = "linux"/*, target_os = "windows" */)))]
compile_error!("Unsupported OS");
