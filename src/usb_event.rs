#[cfg(target_os = "linux")]
extern crate udev;

// Information gathered from the received USB event
#[derive(Debug)]
pub struct USBEvent {
    pub action: String,
}

impl USBEvent {
    #[cfg(target_os = "linux")]
    pub fn from_uevent(uevent: udev::Event) -> Self {
        USBEvent {
            action: uevent.event_type().to_string(),
        }
    }
}
